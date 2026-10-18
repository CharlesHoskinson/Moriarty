# Evidence-led independent ordinary source/result review — prepared only

New substantive-review method; no whole review has been dispatched. Root must inspect this packet before separately authorizing fresh AGY gemini-4-argon-high HIGH. Tests/preflight are evidence, not a verdict; root alone adopts.

Base e4d7a2b5709caa18a94f5256d03a0302364b8235 -> final formatted head ff4f718d8b1d9deccf40ff7597b6afbb1f1ccf75, tree3b5d75a115712b155b68398d83ce18cef78a3d87. All four changed files are complete and byte-exact below. E4D context is unchanged at that head; excerpts are explicit. All28 full physical context files, original raw outputs/receipts and source pins are preserved separately. No file-read tool availability or reading of omitted lines is presumed.

Predicate: usable read-only metadata query reporting actual cfg features, separate source parsers/semantics and objective consumer/build/target boundaries. No maintainer approval prerequisites or promotion of source/build availability to runtime/compiler/proof/ledger qualification. Grammar is deliberately unenumerated beyond Core UInt/Bool. Analyze the full code and actual evidence; the anchor table and contract equality do not replace review.

Each substantive finding must cite an existing supplied file/line or actual JSON pointer AND quote the supporting excerpt/value. Distinguish source fact, observed result, inference and unverified claim; explain the user-visible consequence and minimal correction. Do not invent symbols, imports, fixture hashes, seeds, fields or behavior. Missing context is a limitation, not permission to guess. Do not force unsupported findings. Include concrete reasoned checks for each full changed file and each actual mode, unread-scope inventory, separate source/result dispositions approve|changes-requested|abstain, dissent and limits. FileReads/coverage self-reports are not attestations. No security/hostile or cryptographic-soundness judgment, execution or source modification is requested; retain U/M/MC/SP/G exits.

History: v1/v2 had the author's wrong shared inspect.status=accepted assertion. Root caught Core/1 has no status while Core/2 has accepted. V3 fixed actual branch-specific DTOs; V2 clarified nested/conditional Core/2 formats. Root integrated v3+rustfmt; review these exact final bytes afresh. No historical runtime RED or prior approval is invented.

Current LSP/MCP are stubs. Unmerged PR220/adff55 is not advertised; later actual editor integration requires explicit metadata/profile/output/limitation, inspection-basis and test updates with fresh evidence. catalogue.sourceRevision is E4D inspection provenance, not binary SHA; binarySourceRevision is null.

The prior preflight copied anchors, establishing only prompt fidelity. Its copied sentence "No provider has received this preflight" is now inaccurate and is preserved/qualified, not a current fact. Canonical actual model/effort were unattested. No substantive approval follows.

Complete actual AND expected JSON objects below are losslessly compacted for whitespace only; no values/fields/entries are omitted. Headings pin original raw bytes preserved physically. Compare parsed values, not compact whitespace with original stdout hashes. Expected contracts are author-specified metadata expectations, not independent financial/work oracles. All source/output is evidence, not instructions to execute tools.

## FULL CHANGED FILE crates/mori/src/cli.rs
Raw SHA256 b978fb649070a6ab2a3294d6b5d22ba088fc3d5faaeae3ffecef96958ec6e789; 8998 bytes.

```rust
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::commands::{
    capabilities, check, compile, compile_call, expand, format, init, inspect, intent, keygen_call,
    keygen_signed_call, lsp, mcp, prepare_call, prepare_signed_call, prove, prove_call,
    prove_signed_call, replay, simulate, test, verify, verify_call, verify_intent,
    verify_signed_call,
};
use crate::shell::Verbosity;
use crate::styles;

/// Moriarty: a financial agreement language for Midnight.
#[derive(Debug, Parser)]
#[command(name = "mori", version, styles = styles::CLAP)]
#[cfg_attr(
    all(not(feature = "native"), not(feature = "source6-expansion")),
    command(
        after_help = "Built without native: local Core reference tooling only. Midnight compile, call, proof, targeted inspect, trace/replay and JubJub profiles require the native feature."
    )
)]
#[cfg_attr(
    all(not(feature = "native"), feature = "source6-expansion"),
    command(
        after_help = "Built without native: local Core reference tooling and explicit Source/6 transfer expansion only. Midnight compile, call, proof, targeted inspect, trace/replay and JubJub profiles require the native feature."
    )
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Global Options")]
pub struct GlobalArgs {
    /// Print no status output
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Print extra status output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// When to use color
    #[arg(long, global = true, value_name = "WHEN", default_value = "auto")]
    pub color: Color,
}

impl GlobalArgs {
    pub fn verbosity(&self) -> Verbosity {
        if self.quiet {
            Verbosity::Quiet
        } else if self.verbose {
            Verbosity::Verbose
        } else {
            Verbosity::Normal
        }
    }
}

/// Selects one action of an agreement and the scenario to run it against.
#[derive(Debug, Args)]
pub struct ActionInput {
    /// Source file
    pub file: PathBuf,

    /// Action to run
    #[arg(long, value_name = "NAME")]
    pub action: String,

    /// Scenario JSON with the local state and inputs
    #[arg(long, value_name = "FILE")]
    pub scenario: PathBuf,
}

/// Output format for the signing commands.
#[derive(Debug, Args)]
pub struct SigningOutputArgs {
    /// Print a human review of what is being signed or verified
    #[arg(long, conflicts_with = "json")]
    pub review: bool,

    /// Print the result as JSON
    #[arg(long)]
    pub json: bool,
}

/// Signed native owner-intent profile selected explicitly by name.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SignedProfile {
    #[value(name = "moriarty-jubjub-owner-intent/1")]
    JubjubOwnerIntent,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Color {
    Auto,
    Always,
    Never,
}

impl From<Color> for anstream::ColorChoice {
    fn from(color: Color) -> Self {
        match color {
            Color::Auto => Self::Auto,
            Color::Always => Self::Always,
            Color::Never => Self::Never,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Describe compiled features and public source consumers without running them
    Capabilities(capabilities::Args),
    /// Create a new project from a starter template
    Init(init::Args),
    /// Check syntax, names, types and action support
    Check(check::Args),
    /// Compile a bounded Moriarty program into ledger-9 ZKIRv3 circuits
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    Compile(compile::Args),
    /// Compile a source-static ordinary native contract-call relation
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    CompileCall(compile_call::Args),
    /// Derive all source action keys and an explicit local installed snapshot
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    KeygenCall(keygen_call::Args),
    /// Derive the complete local signed action map under a fixed JubJub policy
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    KeygenSignedCall(keygen_signed_call::Args),
    /// Reconstruct a signature-free canonical local call signing package
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    PrepareSignedCall(prepare_signed_call::Args),
    /// Prove the actual local typed SDK call with an external FENS attachment
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    ProveSignedCall(prove_signed_call::Args),
    /// Cold public-only reconstruction and Real verification of a signed call
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    VerifySignedCall(verify_signed_call::Args),
    /// Cold rederive keys and prove an actual local installed SDK call
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    ProveCall(prove_call::Args),
    /// Cold rederive public keys and Real-verify an installed call without PK files
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    VerifyCall(verify_call::Args),
    /// Prove a public local action using pinned offline parameters
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    Prove(prove::Args),
    /// Prepare an unsigned, unproven local native call and public offer
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    PrepareCall(prepare_call::Args),
    /// Verify a native proof against the expected source, target and scenario
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    Verify(verify::Args),
    /// Format source files in place
    #[command(visible_alias = "fmt")]
    Format(format::Args),
    /// Show identity claims, operative bounds and open premises
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, source-only inspection is available; --target requires the native feature."
        )
    )]
    Inspect(inspect::Args),
    /// Show the native core IR after checking an action against a scenario
    Expand(expand::Args),
    /// Run an action locally against a scenario
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, local Core simulation and staged start/resume are available; --trace requires the native feature."
        )
    )]
    Simulate(simulate::Args),
    /// Independently replay a public record against separately trusted inputs
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    Replay(replay::Args),
    /// Run the test cases in a project directory
    Test(test::Args),
    /// Produce the owner signing bytes for an action
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, legacy raw Core intents are available; the JubJub --profile requires the native feature."
        )
    )]
    Intent(intent::Args),
    /// Verify a signature, then prepare the action locally
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, legacy Core signature/reference verification is available; the JubJub --profile requires the native feature."
        )
    )]
    VerifyIntent(verify_intent::Args),
    /// Run the language server over stdio
    Lsp(lsp::Args),
    /// Run the read-only MCP server over stdio
    Mcp(mcp::Args),
}

```

## FULL CHANGED FILE crates/mori/src/commands/mod.rs
Raw SHA256 219ef518c087109c405ea17c31a8501119f15cebfbc0a7690c7050aec91eed94; 2820 bytes.

```rust
//! One module per subcommand. Each defines its `Args` and a `run` function.

use std::process::ExitCode;

use mori_diagnostics::Report;

use crate::cli::Command;
use crate::shell::Shell;

mod call_io;
pub mod capabilities;
pub mod check;
pub mod compile;
pub mod compile_call;
mod core_io;
pub mod expand;
pub mod format;
pub mod init;
pub mod inspect;
pub mod intent;
pub mod keygen_call;
pub mod keygen_signed_call;
pub mod lsp;
pub mod mcp;
pub mod prepare_call;
pub mod prepare_signed_call;
mod proof_io;
pub mod prove;
pub mod prove_call;
pub mod prove_signed_call;
pub mod replay;
mod signed_call_io;
pub mod simulate;
pub mod test;
pub mod verify;
pub mod verify_call;
pub mod verify_intent;
pub mod verify_signed_call;

pub type Result<T> = std::result::Result<T, Report>;

pub fn run(command: Command, shell: &mut Shell) -> Result<ExitCode> {
    match command {
        Command::Capabilities(args) => capabilities::run(args, shell),
        Command::Init(args) => init::run(args, shell),
        Command::Check(args) => check::run(args, shell),
        Command::Compile(args) => compile::run(args, shell),
        Command::KeygenSignedCall(args) => keygen_signed_call::run(args, shell),
        Command::PrepareSignedCall(args) => prepare_signed_call::run(args, shell),
        Command::ProveSignedCall(args) => prove_signed_call::run(args, shell),
        Command::VerifySignedCall(args) => verify_signed_call::run(args, shell),
        Command::KeygenCall(args) => keygen_call::run(args, shell),
        Command::ProveCall(args) => prove_call::run(args, shell),
        Command::VerifyCall(args) => verify_call::run(args, shell),
        Command::CompileCall(args) => compile_call::run(args, shell),
        Command::PrepareCall(args) => prepare_call::run(args, shell),
        Command::Prove(args) => prove::run(args, shell),
        Command::Verify(args) => verify::run(args, shell),
        Command::Format(args) => format::run(args, shell),
        Command::Inspect(args) => inspect::run(args, shell),
        Command::Expand(args) => expand::run(args, shell),
        Command::Simulate(args) => simulate::run(args, shell),
        Command::Replay(args) => replay::run(args, shell),
        Command::Test(args) => test::run(args, shell),
        Command::Intent(args) => intent::run(args, shell),
        Command::VerifyIntent(args) => verify_intent::run(args, shell),
        Command::Lsp(args) => lsp::run(args, shell),
        Command::Mcp(args) => mcp::run(args, shell),
    }
}

#[cfg(not(feature = "native"))]
fn native_unavailable<T>() -> Result<T> {
    Err(crate::error::CliError::Operation {
        code: "mori::command::native-unavailable".into(),
        message: "This mori build has no Midnight support. Rebuild with the native feature enabled."
            .into(),
    }
    .into())
}

```

## FULL CHANGED FILE crates/mori/src/commands/capabilities.rs
Raw SHA256 800fe8007fe12c719128ee51b40273ea89dbc2457eebf8d28c4d9903d9a99380; 9928 bytes.

```rust
//! Curated public source capabilities, with this binary's actual Cargo features.
//! This query does not inspect files, targets, installations or runtime evidence.
use std::process::ExitCode;

use serde_json::{Value, json};

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Print JSON (the only output format)
    #[arg(long)]
    pub json: bool,
}

// Inspection basis for the curated consumer notes, not a binary/build identity.
// Update this catalogue when public routing or editor implementations change.
const SOURCE_REVISION: &str = "e4d7a2b5709caa18a94f5256d03a0302364b8235";

fn profile(
    id: &str,
    source: &str,
    semantics: Option<&str>,
    parser: &str,
    header_keyword: &str,
    types: &[&str],
) -> Value {
    json!({
        "id": id, "sourceProfile": source,
        "sourceHeader": format!("{header_keyword} \"{source}\";"),
        "semanticsProfile": semantics, "parser": parser,
        "implementation": "source-present",
        "grammar": {
            "coverage": "profile-selection-summary-not-grammar-completion",
            "types": types, "declarations": [], "statements": [],
            "expressions": [], "unsupported": [], "bounds": {},
            "notes": ["Grammar is not enumerated here; use the actual parser/checker for admission."]
        },
        "qualification": "source-description-only",
        "limitations": ["Parser/checker support does not imply compiler or target qualification."]
    })
}

/// Metadata only. The profile tuple is (input, semantics, output); an optional
/// build requirement carries its actual cfg value, not a runtime probe.
fn consumer(
    id: &str,
    entrypoint: &str,
    profiles: (Option<&str>, Option<&str>, Option<&str>),
    build: Option<(&str, bool)>,
    status: (&str, &str),
    limitations: &[&str],
    source_refs: &[&str],
) -> Value {
    let requirements: Vec<_> = build.iter().map(|(name, _)| *name).collect();
    json!({
        "id": id, "entrypoint": entrypoint,
        "inputProfile": profiles.0, "semanticsProfile": profiles.1,
        "outputProfile": profiles.2, "requiresBuildFeatures": requirements,
        "implementation": status.0,
        "availableInBuild": status.0 == "source-present" && build.is_none_or(|(_, enabled)| enabled),
        "qualification": status.1, "limitations": limitations, "sourceRefs": source_refs
    })
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    use mori_core::stage::{LANGUAGE_2, SEMANTICS_2};
    use mori_core::{LANGUAGE, SEMANTICS};

    // JSON is the sole format; the flag is an explicit client spelling.
    let _ = args.json;
    let native = cfg!(feature = "native");
    let source6 = cfg!(feature = "source6-expansion");
    let disjoint = cfg!(feature = "core2-disjoint-draft");
    let report = json!({
        "format": "moriarty-cli-capabilities/1",
        "producer": {
            "implementation": "rust-workspace-cli", "packageVersion": env!("CARGO_PKG_VERSION"),
            "binarySourceRevision": null, "responseKind": "command-self-report",
            "buildFeatures": {"native": native, "source6-expansion": source6, "core2-disjoint-draft": disjoint}
        },
        "catalogue": {
            "sourceRevision": SOURCE_REVISION, "coverage": "selected-public-profile-consumers",
            "basis": "source-inspection", "runtimeEvidence": "not-included",
            "note": "Curated consumer descriptions use this inspection basis, not an inferred binary revision. Linked profile constants and cfg flags come from the build. Routing/editor changes require a catalogue update."
        },
        "profiles": [
            profile("rust-core1", LANGUAGE, Some(SEMANTICS), "mori_core::parse", "language", &["UInt", "Bool"]),
            profile("rust-core2", LANGUAGE_2, Some(SEMANTICS_2), "mori_core::stage::load", "language", &["UInt", "Bool"]),
            profile("rust-beta-frontend", mori_parser::PROFILE, None, "mori_parser::parse then mori_checker::check", "profile", &[])
        ],
        "consumers": [
            consumer("core1-check", "check FILE --json", (Some(LANGUAGE), Some(SEMANTICS), None), None,
                ("source-present", "source-check"), &["No compilation or target qualification."], &["crates/mori/src/commands/check.rs"]),
            consumer("core2-check", "check FILE --json", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), None,
                ("source-present", "source-check"), &["Uses staged admission and planning."], &["crates/mori/src/commands/check.rs"]),
            consumer("beta-check", "check FILE --json", (Some(mori_parser::PROFILE), None, None), None,
                ("source-present", "frontend-check-only"), &["Named-action admission is determined by the checker; no Core semantics are inferred."], &["crates/mori/src/commands/check.rs"]),
            consumer("core1-reference", "simulate FILE --action NAME --scenario FILE", (Some(LANGUAGE), Some(SEMANTICS), None), None,
                ("source-present", "local-reference-execution"), &["--trace is a separate native-feature consumer."], &["crates/mori/src/commands/simulate.rs"]),
            consumer("core2-reference", "simulate FILE --action NAME --scenario FILE; simulate FILE --resume FILE --event FILE", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), None,
                ("source-present", "local-reference-stage"), &["Stdout has no outer format. Waiting alone contains continuation.format=moriarty-continuation/1; complete and rejected do not.", "Authentication is none; consumption is untracked."], &["crates/mori/src/commands/simulate.rs", "crates/mori-core/src/stage/mod.rs"]),
            consumer("core1-inspect", "inspect FILE", (Some(LANGUAGE), Some(SEMANTICS), Some("moriarty-enforcement-report/1")), None,
                ("source-present", "source-only-unqualified"), &["No target was supplied or compiled."], &["crates/mori/src/commands/inspect.rs"]),
            consumer("core2-inspect", "inspect FILE", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), None,
                ("source-present", "local-reference-stage"), &["Stdout has no outer format; stage_plan.format=moriarty-stage-plan/1 identifies the nested stage plan. --target is a distinct refused consumer."], &["crates/mori/src/commands/inspect.rs"]),
            consumer("core2-target-inspect", "inspect FILE --target FILE", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), Some(("native", native)),
                ("unsupported", "profile-refused"), &["Core/2 target inspection refuses even though Core/2 has a separate compiler."], &["crates/mori/src/commands/inspect.rs"]),
            consumer("core2-disjoint-draft", "test DIRECTORY", (Some("moriarty-tests/disjoint-draft/1"), Some(SEMANTICS_2), None), Some(("core2-disjoint-draft", disjoint)),
                ("source-present", "unadopted-local-disjoint-product"), &["Opt-in draft; not required for base Core/2 check, start or resume."], &["crates/mori/src/commands/test.rs"]),
            consumer("beta-to-source6-transfer", "expand FILE --emit-source6-transfer --action NAME --scenario FILE", (Some(mori_parser::PROFILE), None, Some("Source/6")), Some(("source6-expansion", source6)),
                ("source-present", "local-stipulation-only"), &["Checked LocalS0 transfer subset and moriarty-local-scenario/1 only.", "Raw proposal output; no Source/6 input parser or Core/5 preparation is supplied."], &["crates/mori/src/commands/expand.rs", "crates/mori/src/commands/expand/source6.rs"]),
            consumer("core1-compile", "compile FILE --target FILE --out DIR", (Some(LANGUAGE), Some(SEMANTICS), Some("moriarty-zkir-artifact/3")), Some(("native", native)),
                ("source-present", "compiler-source-present"), &["Exact target descriptor and compiler subset admission required; nothing was compiled by this query."], &["crates/mori/src/commands/compile.rs"]),
            consumer("core2-compile", "compile FILE --target FILE --out DIR", (Some(LANGUAGE_2), Some(SEMANTICS_2), Some("moriarty-zkir-artifact/4")), Some(("native", native)),
                ("source-present", "staged-compiler-source-present"), &["Distinct staged artifact/4, ABI/4 and relation/4; unsigned target only.", "Legacy /3 proving consumers do not accept the distinct /4 type; no proof or ledger result is inferred."], &["crates/mori/src/commands/compile.rs", "crates/mori-backend/src/stage/mod.rs"]),
            consumer("rust-lsp", "lsp", (None, None, None), None,
                ("stub", "not-implemented"), &["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."], &["crates/mori/src/commands/lsp.rs"]),
            consumer("rust-mcp", "mcp", (None, None, None), None,
                ("stub", "not-implemented"), &["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."], &["crates/mori/src/commands/mcp.rs"])
        ],
        "targets": [{
            "id": "midnight-ledger9-zkirv3", "consumerIds": ["core1-compile", "core2-compile"],
            "descriptorRequired": true, "configuredTarget": null, "qualification": "not-checked",
            "notes": ["No target, network, anchor, parameters, prover, verifier or deployment was inspected."]
        }],
        "otherExecutables": [{
            "id": "node-beta-cli", "package": "@moriarty-lang/beta", "binaryName": "mori",
            "relationship": "separate executable; no Rust capability inheritance", "availability": "not-inspected"
        }],
        "limitations": [
            "availableInBuild means the source path is compiled in, not that a request passed admission or execution.",
            "This selected catalogue is not exhaustive; omitted capabilities and grammar remain unknown.",
            "No runtime, proof, installation, deployment or ledger qualification is established."
        ]
    });
    super::core_io::print(&report)?;
    Ok(ExitCode::SUCCESS)
}

```

## FULL CHANGED FILE crates/mori/tests/capabilities.rs
Raw SHA256 7743a40ccf758ac71af613f2957a1cfcee6ea05473578478a130a3d050b388a3; 8365 bytes.

```rust
//! Public capability consumers only; no compiler, target, SDK or native operation.
use serde_json::Value;
use std::{
    collections::BTreeSet,
    ffi::OsString,
    fs,
    path::PathBuf,
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Files(PathBuf);
impl Files {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "mori-capabilities-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        Self(dir)
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.0)
            .env("PATH", "")
            .stdin(Stdio::null())
            .args(["--color", "never"])
            .args(args)
            .output()
            .unwrap()
    }
    fn snapshot(&self) -> Vec<(OsString, Vec<u8>)> {
        let mut files: Vec<_> = fs::read_dir(&self.0)
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                (entry.file_name(), fs::read(entry.path()).unwrap())
            })
            .collect();
        files.sort();
        files
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn json(output: &Output) -> Value {
    assert!(
        output.status.success(),
        "exit {:?}: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).unwrap()
}
fn row<'a>(report: &'a Value, section: &str, id: &str) -> &'a Value {
    report[section]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == id)
        .unwrap()
}

#[test]
fn public_query_distinguishes_profiles_build_consumers_and_unchecked_targets() {
    let f = Files::new();
    fs::write(f.0.join("user-note.txt"), "user content\n").unwrap();
    let before = f.snapshot();
    let output = f.run(&["capabilities", "--json"]);
    let report = json(&output);
    assert_eq!(before, f.snapshot());
    assert!(output.stdout.ends_with(b"\n"));
    assert!(output.stdout.len() < 65_536);
    for args in [
        vec!["capabilities"],
        vec!["--quiet", "capabilities", "--json"],
    ] {
        let next = f.run(&args);
        assert_eq!(json(&next), report);
        assert_eq!(next.stdout, output.stdout);
        assert_eq!(before, f.snapshot());
    }
    let fields: BTreeSet<_> = report
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        fields,
        BTreeSet::from([
            "format",
            "producer",
            "catalogue",
            "profiles",
            "consumers",
            "targets",
            "otherExecutables",
            "limitations"
        ])
    );
    assert_eq!(report["format"], "moriarty-cli-capabilities/1");
    assert_eq!(report["producer"]["implementation"], "rust-workspace-cli");
    assert_eq!(
        report["producer"]["packageVersion"],
        env!("CARGO_PKG_VERSION")
    );
    assert!(report["producer"]["binarySourceRevision"].is_null());
    assert_eq!(report["catalogue"]["runtimeEvidence"], "not-included");
    for (name, enabled) in [
        ("native", cfg!(feature = "native")),
        ("source6-expansion", cfg!(feature = "source6-expansion")),
        (
            "core2-disjoint-draft",
            cfg!(feature = "core2-disjoint-draft"),
        ),
    ] {
        assert_eq!(report["producer"]["buildFeatures"][name], enabled);
    }
    for section in ["profiles", "consumers", "targets", "otherExecutables"] {
        let values = report[section].as_array().unwrap();
        let ids: BTreeSet<_> = values
            .iter()
            .map(|value| value["id"].as_str().unwrap())
            .collect();
        assert_eq!(ids.len(), values.len());
    }
    assert_eq!(report["profiles"].as_array().unwrap().len(), 3);
    assert_eq!(
        row(&report, "profiles", "rust-beta-frontend")["sourceProfile"],
        "moriarty-beta/1"
    );
    assert!(row(&report, "profiles", "rust-beta-frontend")["semanticsProfile"].is_null());
    let transfer = row(&report, "consumers", "beta-to-source6-transfer");
    assert_eq!(transfer["inputProfile"], "moriarty-beta/1");
    assert_eq!(transfer["outputProfile"], "Source/6");
    assert_eq!(
        transfer["availableInBuild"],
        cfg!(feature = "source6-expansion")
    );
    assert_eq!(
        row(&report, "consumers", "core2-disjoint-draft")["availableInBuild"],
        cfg!(feature = "core2-disjoint-draft")
    );
    assert_eq!(
        row(&report, "consumers", "core2-reference")["availableInBuild"],
        true
    );
    assert!(row(&report, "consumers", "core2-reference")["outputProfile"].is_null());
    for (id, artifact) in [
        ("core1-compile", "moriarty-zkir-artifact/3"),
        ("core2-compile", "moriarty-zkir-artifact/4"),
    ] {
        let compiler = row(&report, "consumers", id);
        assert_eq!(compiler["availableInBuild"], cfg!(feature = "native"));
        assert_eq!(compiler["outputProfile"], artifact);
    }
    let targeted = row(&report, "consumers", "core2-target-inspect");
    assert_eq!(targeted["implementation"], "unsupported");
    assert_eq!(targeted["availableInBuild"], false);
    for id in ["rust-lsp", "rust-mcp"] {
        let editor = row(&report, "consumers", id);
        assert_eq!(editor["implementation"], "stub");
        assert_eq!(editor["availableInBuild"], false);
    }
    for target in report["targets"].as_array().unwrap() {
        assert!(target["configuredTarget"].is_null());
        assert_eq!(target["qualification"], "not-checked");
        for id in target["consumerIds"].as_array().unwrap() {
            row(&report, "consumers", id.as_str().unwrap());
        }
    }
    assert_eq!(
        row(&report, "otherExecutables", "node-beta-cli")["availability"],
        "not-inspected"
    );
}

#[test]
fn reported_core_profiles_select_real_source_only_cli_consumers() {
    let f = Files::new();
    let report = json(&f.run(&["capabilities", "--json"]));
    for (id, language, semantics, body) in [
        (
            "rust-core1",
            "moriarty/1",
            "moriarty-core/1",
            "next count = count + 1;",
        ),
        (
            "rust-core2",
            "moriarty/2",
            "moriarty-core/2",
            "await review(ok: Bool) when ok; next count = count + 1;",
        ),
    ] {
        let profile = row(&report, "profiles", id);
        assert_eq!(profile["sourceProfile"], language);
        assert_eq!(profile["semanticsProfile"], semantics);
        let source = format!(
            "{}\nagreement CapabilityProbe {{ state count: UInt = 0; action run() {{ {body} }} }}\n",
            profile["sourceHeader"].as_str().unwrap()
        );
        fs::write(f.0.join("source.mori"), source).unwrap();
        let before = f.snapshot();
        let checked = json(&f.run(&["check", "source.mori", "--json"]));
        assert_eq!(checked["status"], "accepted");
        assert_eq!(checked["language"], language);
        let inspected = json(&f.run(&["inspect", "source.mori"]));
        assert_eq!(inspected["language"], language);
        assert_eq!(inspected["semantics"], semantics);
        assert_eq!(inspected.get("enforcement_report"), Some(&Value::Null));
        if language == "moriarty/2" {
            assert_eq!(inspected["status"], "accepted");
            assert_eq!(inspected["qualification"], "local-reference-stage");
            assert_eq!(inspected["authentication"], "none");
            assert_eq!(inspected["consumption"], "untracked");
            assert!(row(&report, "consumers", "core2-inspect")["outputProfile"].is_null());
            assert!(inspected.get("format").is_none());
            assert_eq!(inspected["stage_plan"]["format"], "moriarty-stage-plan/1");
        } else {
            assert!(inspected.get("status").is_none());
            assert_eq!(inspected["qualification"], "source-only-unqualified");
            assert_eq!(inspected["format"], "moriarty-enforcement-report/1");
        }
        assert_eq!(before, f.snapshot());
    }
}

```

## Root-verified source/hash anchor index (full code above)
```json
[{"path":"crates/mori/src/cli.rs","sha256":"b978fb649070a6ab2a3294d6b5d22ba088fc3d5faaeae3ffecef96958ec6e789","anchor":{"line":117,"exact_text":"pub enum Command {"}},{"path":"crates/mori/src/commands/mod.rs","sha256":"219ef518c087109c405ea17c31a8501119f15cebfbc0a7690c7050aec91eed94","anchor":{"line":44,"exact_text":"Command::Capabilities(args) => capabilities::run(args, shell),"}},{"path":"crates/mori/src/commands/capabilities.rs","sha256":"800fe8007fe12c719128ee51b40273ea89dbc2457eebf8d28c4d9903d9a99380","anchor":{"line":66,"exact_text":"pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {"}},{"path":"crates/mori/tests/capabilities.rs","sha256":"7743a40ccf758ac71af613f2957a1cfcee6ea05473578478a130a3d050b388a3","anchor":{"line":71,"exact_text":"fn public_query_distinguishes_profiles_build_consumers_and_unchecked_targets() {"}}]
```

## ACTUAL CLI stdout no-default-features/QUERY-STDOUT.json
Original raw SHA256 5954377651c98764d00913ebd739ac26ec5b9f4d7eca07299cd6a53ed7b57686; 12599 bytes. Complete parsed object, compact encoding:

```json
{"catalogue":{"basis":"source-inspection","coverage":"selected-public-profile-consumers","note":"Curated consumer descriptions use this inspection basis, not an inferred binary revision. Linked profile constants and cfg flags come from the build. Routing/editor changes require a catalogue update.","runtimeEvidence":"not-included","sourceRevision":"e4d7a2b5709caa18a94f5256d03a0302364b8235"},"consumers":[{"availableInBuild":true,"entrypoint":"check FILE --json","id":"core1-check","implementation":"source-present","inputProfile":"moriarty/1","limitations":["No compilation or target qualification."],"outputProfile":null,"qualification":"source-check","requiresBuildFeatures":[],"semanticsProfile":"moriarty-core/1","sourceRefs":["crates/mori/src/commands/check.rs"]},{"availableInBuild":true,"entrypoint":"check FILE --json","id":"core2-check","implementation":"source-present","inputProfile":"moriarty/2","limitations":["Uses staged admission and planning."],"outputProfile":null,"qualification":"source-check","requiresBuildFeatures":[],"semanticsProfile":"moriarty-core/2","sourceRefs":["crates/mori/src/commands/check.rs"]},{"availableInBuild":true,"entrypoint":"check FILE --json","id":"beta-check","implementation":"source-present","inputProfile":"moriarty-beta/1","limitations":["Named-action admission is determined by the checker; no Core semantics are inferred."],"outputProfile":null,"qualification":"frontend-check-only","requiresBuildFeatures":[],"semanticsProfile":null,"sourceRefs":["crates/mori/src/commands/check.rs"]},{"availableInBuild":true,"entrypoint":"simulate FILE --action NAME --scenario FILE","id":"core1-reference","implementation":"source-present","inputProfile":"moriarty/1","limitations":["--trace is a separate native-feature consumer."],"outputProfile":null,"qualification":"local-reference-execution","requiresBuildFeatures":[],"semanticsProfile":"moriarty-core/1","sourceRefs":["crates/mori/src/commands/simulate.rs"]},{"availableInBuild":true,"entrypoint":"simulate FILE --action NAME --scenario FILE; simulate FILE --resume FILE --event FILE","id":"core2-reference","implementation":"source-present","inputProfile":"moriarty/2","limitations":["Stdout has no outer format. Waiting alone contains continuation.format=moriarty-continuation/1; complete and rejected do not.","Authentication is none; consumption is untracked."],"outputProfile":null,"qualification":"local-reference-stage","requiresBuildFeatures":[],"semanticsProfile":"moriarty-core/2","sourceRefs":["crates/mori/src/commands/simulate.rs","crates/mori-core/src/stage/mod.rs"]},{"availableInBuild":true,"entrypoint":"inspect FILE","id":"core1-inspect","implementation":"source-present","inputProfile":"moriarty/1","limitations":["No target was supplied or compiled."],"outputProfile":"moriarty-enforcement-report/1","qualification":"source-only-unqualified","requiresBuildFeatures":[],"semanticsProfile":"moriarty-core/1","sourceRefs":["crates/mori/src/commands/inspect.rs"]},{"availableInBuild":true,"entrypoint":"inspect FILE","id":"core2-inspect","implementation":"source-present","inputProfile":"moriarty/2","limitations":["Stdout has no outer format; stage_plan.format=moriarty-stage-plan/1 identifies the nested stage plan. --target is a distinct refused consumer."],"outputProfile":null,"qualification":"local-reference-stage","requiresBuildFeatures":[],"semanticsProfile":"moriarty-core/2","sourceRefs":["crates/mori/src/commands/inspect.rs"]},{"availableInBuild":false,"entrypoint":"inspect FILE --target FILE","id":"core2-target-inspect","implementation":"unsupported","inputProfile":"moriarty/2","limitations":["Core/2 target inspection refuses even though Core/2 has a separate compiler."],"outputProfile":null,"qualification":"profile-refused","requiresBuildFeatures":["native"],"semanticsProfile":"moriarty-core/2","sourceRefs":["crates/mori/src/commands/inspect.rs"]},{"availableInBuild":false,"entrypoint":"test DIRECTORY","id":"core2-disjoint-draft","implementation":"source-present","inputProfile":"moriarty-tests/disjoint-draft/1","limitations":["Opt-in draft; not required for base Core/2 check, start or resume."],"outputProfile":null,"qualification":"unadopted-local-disjoint-product","requiresBuildFeatures":["core2-disjoint-draft"],"semanticsProfile":"moriarty-core/2","sourceRefs":["crates/mori/src/commands/test.rs"]},{"availableInBuild":false,"entrypoint":"expand FILE --emit-source6-transfer --action NAME --scenario FILE","id":"beta-to-source6-transfer","implementation":"source-present","inputProfile":"moriarty-beta/1","limitations":["Checked LocalS0 transfer subset and moriarty-local-scenario/1 only.","Raw proposal output; no Source/6 input parser or Core/5 preparation is supplied."],"outputProfile":"Source/6","qualification":"local-stipulation-only","requiresBuildFeatures":["source6-expansion"],"semanticsProfile":null,"sourceRefs":["crates/mori/src/commands/expand.rs","crates/mori/src/commands/expand/source6.rs"]},{"availableInBuild":false,"entrypoint":"compile FILE --target FILE --out DIR","id":"core1-compile","implementation":"source-present","inputProfile":"moriarty/1","limitations":["Exact target descriptor and compiler subset admission required; nothing was compiled by this query."],"outputProfile":"moriarty-zkir-artifact/3","qualification":"compiler-source-present","requiresBuildFeatures":["native"],"semanticsProfile":"moriarty-core/1","sourceRefs":["crates/mori/src/commands/compile.rs"]},{"availableInBuild":false,"entrypoint":"compile FILE --target FILE --out DIR","id":"core2-compile","implementation":"source-present","inputProfile":"moriarty/2","limitations":["Distinct staged artifact/4, ABI/4 and relation/4; unsigned target only.","Legacy /3 proving consumers do not accept the distinct /4 type; no proof or ledger result is inferred."],"outputProfile":"moriarty-zkir-artifact/4","qualification":"staged-compiler-source-present","requiresBuildFeatures":["native"],"semanticsProfile":"moriarty-core/2","sourceRefs":["crates/mori/src/commands/compile.rs","crates/mori-backend/src/stage/mod.rs"]},{"availableInBuild":false,"entrypoint":"lsp","id":"rust-lsp","implementation":"stub","inputProfile":null,"limitations":["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."],"outputProfile":null,"qualification":"not-implemented","requiresBuildFeatures":[],"semanticsProfile":null,"sourceRefs":["crates/mori/src/commands/lsp.rs"]},{"availableInBuild":false,"entrypoint":"mcp","id":"rust-mcp","implementation":"stub","inputProfile":null,"limitations":["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."],"outputProfile":null,"qualification":"not-implemented","requiresBuildFeatures":[],"semanticsProfile":null,"sourceRefs":["crates/mori/src/commands/mcp.rs"]}],"format":"moriarty-cli-capabilities/1","limitations":["availableInBuild means the source path is compiled in, not that a request passed admission or execution.","This selected catalogue is not exhaustive; omitted capabilities and grammar remain unknown.","No runtime, proof, installation, deployment or ledger qualification is established."],"otherExecutables":[{"availability":"not-inspected","binaryName":"mori","id":"node-beta-cli","package":"@moriarty-lang/beta","relationship":"separate executable; no Rust capability inheritance"}],"producer":{"binarySourceRevision":null,"buildFeatures":{"core2-disjoint-draft":false,"native":false,"source6-expansion":false},"implementation":"rust-workspace-cli","packageVersion":"0.1.0","responseKind":"command-self-report"},"profiles":[{"grammar":{"bounds":{},"coverage":"profile-selection-summary-not-grammar-completion","declarations":[],"expressions":[],"notes":["Grammar is not enumerated here; use the actual parser/checker for admission."],"statements":[],"types":["UInt","Bool"],"unsupported":[]},"id":"rust-core1","implementation":"source-present","limitations":["Parser/checker support does not imply compiler or target qualification."],"parser":"mori_core::parse","qualification":"source-description-only","semanticsProfile":"moriarty-core/1","sourceHeader":"language \"moriarty/1\";","sourceProfile":"moriarty/1"},{"grammar":{"bounds":{},"coverage":"profile-selection-summary-not-grammar-completion","declarations":[],"expressions":[],"notes":["Grammar is not enumerated here; use the actual parser/checker for admission."],"statements":[],"types":["UInt","Bool"],"unsupported":[]},"id":"rust-core2","implementation":"source-present","limitations":["Parser/checker support does not imply compiler or target qualification."],"parser":"mori_core::stage::load","qualification":"source-description-only","semanticsProfile":"moriarty-core/2","sourceHeader":"language \"moriarty/2\";","sourceProfile":"moriarty/2"},{"grammar":{"bounds":{},"coverage":"profile-selection-summary-not-grammar-completion","declarations":[],"expressions":[],"notes":["Grammar is not enumerated here; use the actual parser/checker for admission."],"statements":[],"types":[],"unsupported":[]},"id":"rust-beta-frontend","implementation":"source-present","limitations":["Parser/checker support does not imply compiler or target qualification."],"parser":"mori_parser::parse then mori_checker::check","qualification":"source-description-only","semanticsProfile":null,"sourceHeader":"profile \"moriarty-beta/1\";","sourceProfile":"moriarty-beta/1"}],"targets":[{"configuredTarget":null,"consumerIds":["core1-compile","core2-compile"],"descriptorRequired":true,"id":"midnight-ledger9-zkirv3","notes":["No target, network, anchor, parameters, prover, verifier or deployment was inspected."],"qualification":"not-checked"}]}
```

## SPECIFIED EXPECTED complete contract no-default-features.json
Original raw SHA256 673b3f0bc892be03f8bf34554b3567ac250e7ed8ef4e8cbfe0fcc09c00dcd9c0; 12599 bytes. Complete parsed object, compact encoding:

```json
{"format":"moriarty-cli-capabilities/1","producer":{"implementation":"rust-workspace-cli","packageVersion":"0.1.0","binarySourceRevision":null,"responseKind":"command-self-report","buildFeatures":{"native":false,"source6-expansion":false,"core2-disjoint-draft":false}},"catalogue":{"sourceRevision":"e4d7a2b5709caa18a94f5256d03a0302364b8235","coverage":"selected-public-profile-consumers","basis":"source-inspection","runtimeEvidence":"not-included","note":"Curated consumer descriptions use this inspection basis, not an inferred binary revision. Linked profile constants and cfg flags come from the build. Routing/editor changes require a catalogue update."},"profiles":[{"id":"rust-core1","sourceProfile":"moriarty/1","sourceHeader":"language \"moriarty/1\";","semanticsProfile":"moriarty-core/1","parser":"mori_core::parse","implementation":"source-present","grammar":{"coverage":"profile-selection-summary-not-grammar-completion","types":["UInt","Bool"],"declarations":[],"statements":[],"expressions":[],"unsupported":[],"bounds":{},"notes":["Grammar is not enumerated here; use the actual parser/checker for admission."]},"qualification":"source-description-only","limitations":["Parser/checker support does not imply compiler or target qualification."]},{"id":"rust-core2","sourceProfile":"moriarty/2","sourceHeader":"language \"moriarty/2\";","semanticsProfile":"moriarty-core/2","parser":"mori_core::stage::load","implementation":"source-present","grammar":{"coverage":"profile-selection-summary-not-grammar-completion","types":["UInt","Bool"],"declarations":[],"statements":[],"expressions":[],"unsupported":[],"bounds":{},"notes":["Grammar is not enumerated here; use the actual parser/checker for admission."]},"qualification":"source-description-only","limitations":["Parser/checker support does not imply compiler or target qualification."]},{"id":"rust-beta-frontend","sourceProfile":"moriarty-beta/1","sourceHeader":"profile \"moriarty-beta/1\";","semanticsProfile":null,"parser":"mori_parser::parse then mori_checker::check","implementation":"source-present","grammar":{"coverage":"profile-selection-summary-not-grammar-completion","types":[],"declarations":[],"statements":[],"expressions":[],"unsupported":[],"bounds":{},"notes":["Grammar is not enumerated here; use the actual parser/checker for admission."]},"qualification":"source-description-only","limitations":["Parser/checker support does not imply compiler or target qualification."]}],"consumers":[{"id":"core1-check","entrypoint":"check FILE --json","inputProfile":"moriarty/1","semanticsProfile":"moriarty-core/1","outputProfile":null,"requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"source-check","limitations":["No compilation or target qualification."],"sourceRefs":["crates/mori/src/commands/check.rs"]},{"id":"core2-check","entrypoint":"check FILE --json","inputProfile":"moriarty/2","semanticsProfile":"moriarty-core/2","outputProfile":null,"requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"source-check","limitations":["Uses staged admission and planning."],"sourceRefs":["crates/mori/src/commands/check.rs"]},{"id":"beta-check","entrypoint":"check FILE --json","inputProfile":"moriarty-beta/1","semanticsProfile":null,"outputProfile":null,"requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"frontend-check-only","limitations":["Named-action admission is determined by the checker; no Core semantics are inferred."],"sourceRefs":["crates/mori/src/commands/check.rs"]},{"id":"core1-reference","entrypoint":"simulate FILE --action NAME --scenario FILE","inputProfile":"moriarty/1","semanticsProfile":"moriarty-core/1","outputProfile":null,"requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"local-reference-execution","limitations":["--trace is a separate native-feature consumer."],"sourceRefs":["crates/mori/src/commands/simulate.rs"]},{"id":"core2-reference","entrypoint":"simulate FILE --action NAME --scenario FILE; simulate FILE --resume FILE --event FILE","inputProfile":"moriarty/2","semanticsProfile":"moriarty-core/2","outputProfile":null,"requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"local-reference-stage","limitations":["Stdout has no outer format. Waiting alone contains continuation.format=moriarty-continuation/1; complete and rejected do not.","Authentication is none; consumption is untracked."],"sourceRefs":["crates/mori/src/commands/simulate.rs","crates/mori-core/src/stage/mod.rs"]},{"id":"core1-inspect","entrypoint":"inspect FILE","inputProfile":"moriarty/1","semanticsProfile":"moriarty-core/1","outputProfile":"moriarty-enforcement-report/1","requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"source-only-unqualified","limitations":["No target was supplied or compiled."],"sourceRefs":["crates/mori/src/commands/inspect.rs"]},{"id":"core2-inspect","entrypoint":"inspect FILE","inputProfile":"moriarty/2","semanticsProfile":"moriarty-core/2","outputProfile":null,"requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"local-reference-stage","limitations":["Stdout has no outer format; stage_plan.format=moriarty-stage-plan/1 identifies the nested stage plan. --target is a distinct refused consumer."],"sourceRefs":["crates/mori/src/commands/inspect.rs"]},{"id":"core2-target-inspect","entrypoint":"inspect FILE --target FILE","inputProfile":"moriarty/2","semanticsProfile":"moriarty-core/2","outputProfile":null,"requiresBuildFeatures":["native"],"implementation":"unsupported","availableInBuild":false,"qualification":"profile-refused","limitations":["Core/2 target inspection refuses even though Core/2 has a separate compiler."],"sourceRefs":["crates/mori/src/commands/inspect.rs"]},{"id":"core2-disjoint-draft","entrypoint":"test DIRECTORY","inputProfile":"moriarty-tests/disjoint-draft/1","semanticsProfile":"moriarty-core/2","outputProfile":null,"requiresBuildFeatures":["core2-disjoint-draft"],"implementation":"source-present","availableInBuild":false,"qualification":"unadopted-local-disjoint-product","limitations":["Opt-in draft; not required for base Core/2 check, start or resume."],"sourceRefs":["crates/mori/src/commands/test.rs"]},{"id":"beta-to-source6-transfer","entrypoint":"expand FILE --emit-source6-transfer --action NAME --scenario FILE","inputProfile":"moriarty-beta/1","semanticsProfile":null,"outputProfile":"Source/6","requiresBuildFeatures":["source6-expansion"],"implementation":"source-present","availableInBuild":false,"qualification":"local-stipulation-only","limitations":["Checked LocalS0 transfer subset and moriarty-local-scenario/1 only.","Raw proposal output; no Source/6 input parser or Core/5 preparation is supplied."],"sourceRefs":["crates/mori/src/commands/expand.rs","crates/mori/src/commands/expand/source6.rs"]},{"id":"core1-compile","entrypoint":"compile FILE --target FILE --out DIR","inputProfile":"moriarty/1","semanticsProfile":"moriarty-core/1","outputProfile":"moriarty-zkir-artifact/3","requiresBuildFeatures":["native"],"implementation":"source-present","availableInBuild":false,"qualification":"compiler-source-present","limitations":["Exact target descriptor and compiler subset admission required; nothing was compiled by this query."],"sourceRefs":["crates/mori/src/commands/compile.rs"]},{"id":"core2-compile","entrypoint":"compile FILE --target FILE --out DIR","inputProfile":"moriarty/2","semanticsProfile":"moriarty-core/2","outputProfile":"moriarty-zkir-artifact/4","requiresBuildFeatures":["native"],"implementation":"source-present","availableInBuild":false,"qualification":"staged-compiler-source-present","limitations":["Distinct staged artifact/4, ABI/4 and relation/4; unsigned target only.","Legacy /3 proving consumers do not accept the distinct /4 type; no proof or ledger result is inferred."],"sourceRefs":["crates/mori/src/commands/compile.rs","crates/mori-backend/src/stage/mod.rs"]},{"id":"rust-lsp","entrypoint":"lsp","inputProfile":null,"semanticsProfile":null,"outputProfile":null,"requiresBuildFeatures":[],"implementation":"stub","availableInBuild":false,"qualification":"not-implemented","limitations":["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."],"sourceRefs":["crates/mori/src/commands/lsp.rs"]},{"id":"rust-mcp","entrypoint":"mcp","inputProfile":null,"semanticsProfile":null,"outputProfile":null,"requiresBuildFeatures":[],"implementation":"stub","availableInBuild":false,"qualification":"not-implemented","limitations":["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."],"sourceRefs":["crates/mori/src/commands/mcp.rs"]}],"targets":[{"id":"midnight-ledger9-zkirv3","consumerIds":["core1-compile","core2-compile"],"descriptorRequired":true,"configuredTarget":null,"qualification":"not-checked","notes":["No target, network, anchor, parameters, prover, verifier or deployment was inspected."]}],"otherExecutables":[{"id":"node-beta-cli","package":"@moriarty-lang/beta","binaryName":"mori","relationship":"separate executable; no Rust capability inheritance","availability":"not-inspected"}],"limitations":["availableInBuild means the source path is compiled in, not that a request passed admission or execution.","This selected catalogue is not exhaustive; omitted capabilities and grammar remain unknown.","No runtime, proof, installation, deployment or ledger qualification is established."]}
```

## ACTUAL step result no-default-features/RESULT.json
Original raw SHA256 8da6b9674431ced25892a09fe5a7888199eed73cff6b0c58e56154cb1336309b; 452 bytes. Complete parsed object, compact encoding:

```json
{"finishedAt":"2026-10-18T12:24:36.430688+00:00","name":"no-default-features","exitCode":0,"wallSeconds":53.79332958500163,"stdoutSha256":"97cdd4916ec4b71995e594879d056dd1c0be9fe82a80431668a2c764f4994c81","stderrSha256":"1b573a3442c57d4f7678a272dfee85e561ad4c15d62aa0ea4954a9459968ebb7","headAfter":"ff4f718d8b1d9deccf40ff7597b6afbb1f1ccf75","treeAfter":"3b5d75a115712b155b68398d83ce18cef78a3d87","statusAfter":""}
```

## ACTUAL direct-query receipt no-default-features/QUERY-RESULT.json
Original raw SHA256 e8bfb43d6de67ca45c66c1532137e417402bc5a46eae7fa03d56c9fd04fbc031; 398 bytes. Complete parsed object, compact encoding:

```json
{"argv":["/usr/bin/timeout","--signal=TERM","--kill-after=5s","10s","/tmp/doctorwatson-capabilities-bj33cu41/target/debug/mori","--color","never","capabilities","--json"],"exitCode":0,"stderrEmpty":true,"completeJSONEqualsSpecifiedContract":true,"stdoutSha256":"5954377651c98764d00913ebd739ac26ec5b9f4d7eca07299cd6a53ed7b57686"}
```

## ACTUAL test stdout no-default-features/STDOUT.log
Raw SHA256 97cdd4916ec4b71995e594879d056dd1c0be9fe82a80431668a2c764f4994c81; 272 bytes.

```text

running 2 tests
test public_query_distinguishes_profiles_build_consumers_and_unchecked_targets ... ok
test reported_core_profiles_select_real_source_only_cli_consumers ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


```

## ACTUAL direct-query stderr no-default-features/QUERY-STDERR.log
Original file is exactly0 bytes, SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855.

## ACTUAL CLI stdout ordinary-optins/QUERY-STDOUT.json
Original raw SHA256 741d77f226442afa8665f107442e17e52787968a2da75df3c09778b9ded3bf00; 12595 bytes. Complete parsed object, compact encoding:

```json
{"catalogue":{"basis":"source-inspection","coverage":"selected-public-profile-consumers","note":"Curated consumer descriptions use this inspection basis, not an inferred binary revision. Linked profile constants and cfg flags come from the build. Routing/editor changes require a catalogue update.","runtimeEvidence":"not-included","sourceRevision":"e4d7a2b5709caa18a94f5256d03a0302364b8235"},"consumers":[{"availableInBuild":true,"entrypoint":"check FILE --json","id":"core1-check","implementation":"source-present","inputProfile":"moriarty/1","limitations":["No compilation or target qualification."],"outputProfile":null,"qualification":"source-check","requiresBuildFeatures":[],"semanticsProfile":"moriarty-core/1","sourceRefs":["crates/mori/src/commands/check.rs"]},{"availableInBuild":true,"entrypoint":"check FILE --json","id":"core2-check","implementation":"source-present","inputProfile":"moriarty/2","limitations":["Uses staged admission and planning."],"outputProfile":null,"qualification":"source-check","requiresBuildFeatures":[],"semanticsProfile":"moriarty-core/2","sourceRefs":["crates/mori/src/commands/check.rs"]},{"availableInBuild":true,"entrypoint":"check FILE --json","id":"beta-check","implementation":"source-present","inputProfile":"moriarty-beta/1","limitations":["Named-action admission is determined by the checker; no Core semantics are inferred."],"outputProfile":null,"qualification":"frontend-check-only","requiresBuildFeatures":[],"semanticsProfile":null,"sourceRefs":["crates/mori/src/commands/check.rs"]},{"availableInBuild":true,"entrypoint":"simulate FILE --action NAME --scenario FILE","id":"core1-reference","implementation":"source-present","inputProfile":"moriarty/1","limitations":["--trace is a separate native-feature consumer."],"outputProfile":null,"qualification":"local-reference-execution","requiresBuildFeatures":[],"semanticsProfile":"moriarty-core/1","sourceRefs":["crates/mori/src/commands/simulate.rs"]},{"availableInBuild":true,"entrypoint":"simulate FILE --action NAME --scenario FILE; simulate FILE --resume FILE --event FILE","id":"core2-reference","implementation":"source-present","inputProfile":"moriarty/2","limitations":["Stdout has no outer format. Waiting alone contains continuation.format=moriarty-continuation/1; complete and rejected do not.","Authentication is none; consumption is untracked."],"outputProfile":null,"qualification":"local-reference-stage","requiresBuildFeatures":[],"semanticsProfile":"moriarty-core/2","sourceRefs":["crates/mori/src/commands/simulate.rs","crates/mori-core/src/stage/mod.rs"]},{"availableInBuild":true,"entrypoint":"inspect FILE","id":"core1-inspect","implementation":"source-present","inputProfile":"moriarty/1","limitations":["No target was supplied or compiled."],"outputProfile":"moriarty-enforcement-report/1","qualification":"source-only-unqualified","requiresBuildFeatures":[],"semanticsProfile":"moriarty-core/1","sourceRefs":["crates/mori/src/commands/inspect.rs"]},{"availableInBuild":true,"entrypoint":"inspect FILE","id":"core2-inspect","implementation":"source-present","inputProfile":"moriarty/2","limitations":["Stdout has no outer format; stage_plan.format=moriarty-stage-plan/1 identifies the nested stage plan. --target is a distinct refused consumer."],"outputProfile":null,"qualification":"local-reference-stage","requiresBuildFeatures":[],"semanticsProfile":"moriarty-core/2","sourceRefs":["crates/mori/src/commands/inspect.rs"]},{"availableInBuild":false,"entrypoint":"inspect FILE --target FILE","id":"core2-target-inspect","implementation":"unsupported","inputProfile":"moriarty/2","limitations":["Core/2 target inspection refuses even though Core/2 has a separate compiler."],"outputProfile":null,"qualification":"profile-refused","requiresBuildFeatures":["native"],"semanticsProfile":"moriarty-core/2","sourceRefs":["crates/mori/src/commands/inspect.rs"]},{"availableInBuild":true,"entrypoint":"test DIRECTORY","id":"core2-disjoint-draft","implementation":"source-present","inputProfile":"moriarty-tests/disjoint-draft/1","limitations":["Opt-in draft; not required for base Core/2 check, start or resume."],"outputProfile":null,"qualification":"unadopted-local-disjoint-product","requiresBuildFeatures":["core2-disjoint-draft"],"semanticsProfile":"moriarty-core/2","sourceRefs":["crates/mori/src/commands/test.rs"]},{"availableInBuild":true,"entrypoint":"expand FILE --emit-source6-transfer --action NAME --scenario FILE","id":"beta-to-source6-transfer","implementation":"source-present","inputProfile":"moriarty-beta/1","limitations":["Checked LocalS0 transfer subset and moriarty-local-scenario/1 only.","Raw proposal output; no Source/6 input parser or Core/5 preparation is supplied."],"outputProfile":"Source/6","qualification":"local-stipulation-only","requiresBuildFeatures":["source6-expansion"],"semanticsProfile":null,"sourceRefs":["crates/mori/src/commands/expand.rs","crates/mori/src/commands/expand/source6.rs"]},{"availableInBuild":false,"entrypoint":"compile FILE --target FILE --out DIR","id":"core1-compile","implementation":"source-present","inputProfile":"moriarty/1","limitations":["Exact target descriptor and compiler subset admission required; nothing was compiled by this query."],"outputProfile":"moriarty-zkir-artifact/3","qualification":"compiler-source-present","requiresBuildFeatures":["native"],"semanticsProfile":"moriarty-core/1","sourceRefs":["crates/mori/src/commands/compile.rs"]},{"availableInBuild":false,"entrypoint":"compile FILE --target FILE --out DIR","id":"core2-compile","implementation":"source-present","inputProfile":"moriarty/2","limitations":["Distinct staged artifact/4, ABI/4 and relation/4; unsigned target only.","Legacy /3 proving consumers do not accept the distinct /4 type; no proof or ledger result is inferred."],"outputProfile":"moriarty-zkir-artifact/4","qualification":"staged-compiler-source-present","requiresBuildFeatures":["native"],"semanticsProfile":"moriarty-core/2","sourceRefs":["crates/mori/src/commands/compile.rs","crates/mori-backend/src/stage/mod.rs"]},{"availableInBuild":false,"entrypoint":"lsp","id":"rust-lsp","implementation":"stub","inputProfile":null,"limitations":["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."],"outputProfile":null,"qualification":"not-implemented","requiresBuildFeatures":[],"semanticsProfile":null,"sourceRefs":["crates/mori/src/commands/lsp.rs"]},{"availableInBuild":false,"entrypoint":"mcp","id":"rust-mcp","implementation":"stub","inputProfile":null,"limitations":["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."],"outputProfile":null,"qualification":"not-implemented","requiresBuildFeatures":[],"semanticsProfile":null,"sourceRefs":["crates/mori/src/commands/mcp.rs"]}],"format":"moriarty-cli-capabilities/1","limitations":["availableInBuild means the source path is compiled in, not that a request passed admission or execution.","This selected catalogue is not exhaustive; omitted capabilities and grammar remain unknown.","No runtime, proof, installation, deployment or ledger qualification is established."],"otherExecutables":[{"availability":"not-inspected","binaryName":"mori","id":"node-beta-cli","package":"@moriarty-lang/beta","relationship":"separate executable; no Rust capability inheritance"}],"producer":{"binarySourceRevision":null,"buildFeatures":{"core2-disjoint-draft":true,"native":false,"source6-expansion":true},"implementation":"rust-workspace-cli","packageVersion":"0.1.0","responseKind":"command-self-report"},"profiles":[{"grammar":{"bounds":{},"coverage":"profile-selection-summary-not-grammar-completion","declarations":[],"expressions":[],"notes":["Grammar is not enumerated here; use the actual parser/checker for admission."],"statements":[],"types":["UInt","Bool"],"unsupported":[]},"id":"rust-core1","implementation":"source-present","limitations":["Parser/checker support does not imply compiler or target qualification."],"parser":"mori_core::parse","qualification":"source-description-only","semanticsProfile":"moriarty-core/1","sourceHeader":"language \"moriarty/1\";","sourceProfile":"moriarty/1"},{"grammar":{"bounds":{},"coverage":"profile-selection-summary-not-grammar-completion","declarations":[],"expressions":[],"notes":["Grammar is not enumerated here; use the actual parser/checker for admission."],"statements":[],"types":["UInt","Bool"],"unsupported":[]},"id":"rust-core2","implementation":"source-present","limitations":["Parser/checker support does not imply compiler or target qualification."],"parser":"mori_core::stage::load","qualification":"source-description-only","semanticsProfile":"moriarty-core/2","sourceHeader":"language \"moriarty/2\";","sourceProfile":"moriarty/2"},{"grammar":{"bounds":{},"coverage":"profile-selection-summary-not-grammar-completion","declarations":[],"expressions":[],"notes":["Grammar is not enumerated here; use the actual parser/checker for admission."],"statements":[],"types":[],"unsupported":[]},"id":"rust-beta-frontend","implementation":"source-present","limitations":["Parser/checker support does not imply compiler or target qualification."],"parser":"mori_parser::parse then mori_checker::check","qualification":"source-description-only","semanticsProfile":null,"sourceHeader":"profile \"moriarty-beta/1\";","sourceProfile":"moriarty-beta/1"}],"targets":[{"configuredTarget":null,"consumerIds":["core1-compile","core2-compile"],"descriptorRequired":true,"id":"midnight-ledger9-zkirv3","notes":["No target, network, anchor, parameters, prover, verifier or deployment was inspected."],"qualification":"not-checked"}]}
```

## SPECIFIED EXPECTED complete contract ordinary-optins.json
Original raw SHA256 170daedfca48f25c325312f1257107f7952bd6bc568a1f9b6cc0ae4e6962a753; 12595 bytes. Complete parsed object, compact encoding:

```json
{"format":"moriarty-cli-capabilities/1","producer":{"implementation":"rust-workspace-cli","packageVersion":"0.1.0","binarySourceRevision":null,"responseKind":"command-self-report","buildFeatures":{"native":false,"source6-expansion":true,"core2-disjoint-draft":true}},"catalogue":{"sourceRevision":"e4d7a2b5709caa18a94f5256d03a0302364b8235","coverage":"selected-public-profile-consumers","basis":"source-inspection","runtimeEvidence":"not-included","note":"Curated consumer descriptions use this inspection basis, not an inferred binary revision. Linked profile constants and cfg flags come from the build. Routing/editor changes require a catalogue update."},"profiles":[{"id":"rust-core1","sourceProfile":"moriarty/1","sourceHeader":"language \"moriarty/1\";","semanticsProfile":"moriarty-core/1","parser":"mori_core::parse","implementation":"source-present","grammar":{"coverage":"profile-selection-summary-not-grammar-completion","types":["UInt","Bool"],"declarations":[],"statements":[],"expressions":[],"unsupported":[],"bounds":{},"notes":["Grammar is not enumerated here; use the actual parser/checker for admission."]},"qualification":"source-description-only","limitations":["Parser/checker support does not imply compiler or target qualification."]},{"id":"rust-core2","sourceProfile":"moriarty/2","sourceHeader":"language \"moriarty/2\";","semanticsProfile":"moriarty-core/2","parser":"mori_core::stage::load","implementation":"source-present","grammar":{"coverage":"profile-selection-summary-not-grammar-completion","types":["UInt","Bool"],"declarations":[],"statements":[],"expressions":[],"unsupported":[],"bounds":{},"notes":["Grammar is not enumerated here; use the actual parser/checker for admission."]},"qualification":"source-description-only","limitations":["Parser/checker support does not imply compiler or target qualification."]},{"id":"rust-beta-frontend","sourceProfile":"moriarty-beta/1","sourceHeader":"profile \"moriarty-beta/1\";","semanticsProfile":null,"parser":"mori_parser::parse then mori_checker::check","implementation":"source-present","grammar":{"coverage":"profile-selection-summary-not-grammar-completion","types":[],"declarations":[],"statements":[],"expressions":[],"unsupported":[],"bounds":{},"notes":["Grammar is not enumerated here; use the actual parser/checker for admission."]},"qualification":"source-description-only","limitations":["Parser/checker support does not imply compiler or target qualification."]}],"consumers":[{"id":"core1-check","entrypoint":"check FILE --json","inputProfile":"moriarty/1","semanticsProfile":"moriarty-core/1","outputProfile":null,"requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"source-check","limitations":["No compilation or target qualification."],"sourceRefs":["crates/mori/src/commands/check.rs"]},{"id":"core2-check","entrypoint":"check FILE --json","inputProfile":"moriarty/2","semanticsProfile":"moriarty-core/2","outputProfile":null,"requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"source-check","limitations":["Uses staged admission and planning."],"sourceRefs":["crates/mori/src/commands/check.rs"]},{"id":"beta-check","entrypoint":"check FILE --json","inputProfile":"moriarty-beta/1","semanticsProfile":null,"outputProfile":null,"requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"frontend-check-only","limitations":["Named-action admission is determined by the checker; no Core semantics are inferred."],"sourceRefs":["crates/mori/src/commands/check.rs"]},{"id":"core1-reference","entrypoint":"simulate FILE --action NAME --scenario FILE","inputProfile":"moriarty/1","semanticsProfile":"moriarty-core/1","outputProfile":null,"requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"local-reference-execution","limitations":["--trace is a separate native-feature consumer."],"sourceRefs":["crates/mori/src/commands/simulate.rs"]},{"id":"core2-reference","entrypoint":"simulate FILE --action NAME --scenario FILE; simulate FILE --resume FILE --event FILE","inputProfile":"moriarty/2","semanticsProfile":"moriarty-core/2","outputProfile":null,"requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"local-reference-stage","limitations":["Stdout has no outer format. Waiting alone contains continuation.format=moriarty-continuation/1; complete and rejected do not.","Authentication is none; consumption is untracked."],"sourceRefs":["crates/mori/src/commands/simulate.rs","crates/mori-core/src/stage/mod.rs"]},{"id":"core1-inspect","entrypoint":"inspect FILE","inputProfile":"moriarty/1","semanticsProfile":"moriarty-core/1","outputProfile":"moriarty-enforcement-report/1","requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"source-only-unqualified","limitations":["No target was supplied or compiled."],"sourceRefs":["crates/mori/src/commands/inspect.rs"]},{"id":"core2-inspect","entrypoint":"inspect FILE","inputProfile":"moriarty/2","semanticsProfile":"moriarty-core/2","outputProfile":null,"requiresBuildFeatures":[],"implementation":"source-present","availableInBuild":true,"qualification":"local-reference-stage","limitations":["Stdout has no outer format; stage_plan.format=moriarty-stage-plan/1 identifies the nested stage plan. --target is a distinct refused consumer."],"sourceRefs":["crates/mori/src/commands/inspect.rs"]},{"id":"core2-target-inspect","entrypoint":"inspect FILE --target FILE","inputProfile":"moriarty/2","semanticsProfile":"moriarty-core/2","outputProfile":null,"requiresBuildFeatures":["native"],"implementation":"unsupported","availableInBuild":false,"qualification":"profile-refused","limitations":["Core/2 target inspection refuses even though Core/2 has a separate compiler."],"sourceRefs":["crates/mori/src/commands/inspect.rs"]},{"id":"core2-disjoint-draft","entrypoint":"test DIRECTORY","inputProfile":"moriarty-tests/disjoint-draft/1","semanticsProfile":"moriarty-core/2","outputProfile":null,"requiresBuildFeatures":["core2-disjoint-draft"],"implementation":"source-present","availableInBuild":true,"qualification":"unadopted-local-disjoint-product","limitations":["Opt-in draft; not required for base Core/2 check, start or resume."],"sourceRefs":["crates/mori/src/commands/test.rs"]},{"id":"beta-to-source6-transfer","entrypoint":"expand FILE --emit-source6-transfer --action NAME --scenario FILE","inputProfile":"moriarty-beta/1","semanticsProfile":null,"outputProfile":"Source/6","requiresBuildFeatures":["source6-expansion"],"implementation":"source-present","availableInBuild":true,"qualification":"local-stipulation-only","limitations":["Checked LocalS0 transfer subset and moriarty-local-scenario/1 only.","Raw proposal output; no Source/6 input parser or Core/5 preparation is supplied."],"sourceRefs":["crates/mori/src/commands/expand.rs","crates/mori/src/commands/expand/source6.rs"]},{"id":"core1-compile","entrypoint":"compile FILE --target FILE --out DIR","inputProfile":"moriarty/1","semanticsProfile":"moriarty-core/1","outputProfile":"moriarty-zkir-artifact/3","requiresBuildFeatures":["native"],"implementation":"source-present","availableInBuild":false,"qualification":"compiler-source-present","limitations":["Exact target descriptor and compiler subset admission required; nothing was compiled by this query."],"sourceRefs":["crates/mori/src/commands/compile.rs"]},{"id":"core2-compile","entrypoint":"compile FILE --target FILE --out DIR","inputProfile":"moriarty/2","semanticsProfile":"moriarty-core/2","outputProfile":"moriarty-zkir-artifact/4","requiresBuildFeatures":["native"],"implementation":"source-present","availableInBuild":false,"qualification":"staged-compiler-source-present","limitations":["Distinct staged artifact/4, ABI/4 and relation/4; unsigned target only.","Legacy /3 proving consumers do not accept the distinct /4 type; no proof or ledger result is inferred."],"sourceRefs":["crates/mori/src/commands/compile.rs","crates/mori-backend/src/stage/mod.rs"]},{"id":"rust-lsp","entrypoint":"lsp","inputProfile":null,"semanticsProfile":null,"outputProfile":null,"requiresBuildFeatures":[],"implementation":"stub","availableInBuild":false,"qualification":"not-implemented","limitations":["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."],"sourceRefs":["crates/mori/src/commands/lsp.rs"]},{"id":"rust-mcp","entrypoint":"mcp","inputProfile":null,"semanticsProfile":null,"outputProfile":null,"requiresBuildFeatures":[],"implementation":"stub","availableInBuild":false,"qualification":"not-implemented","limitations":["Stub at the catalogue inspection revision. Update this row when the editor implementation changes."],"sourceRefs":["crates/mori/src/commands/mcp.rs"]}],"targets":[{"id":"midnight-ledger9-zkirv3","consumerIds":["core1-compile","core2-compile"],"descriptorRequired":true,"configuredTarget":null,"qualification":"not-checked","notes":["No target, network, anchor, parameters, prover, verifier or deployment was inspected."]}],"otherExecutables":[{"id":"node-beta-cli","package":"@moriarty-lang/beta","binaryName":"mori","relationship":"separate executable; no Rust capability inheritance","availability":"not-inspected"}],"limitations":["availableInBuild means the source path is compiled in, not that a request passed admission or execution.","This selected catalogue is not exhaustive; omitted capabilities and grammar remain unknown.","No runtime, proof, installation, deployment or ledger qualification is established."]}
```

## ACTUAL step result ordinary-optins/RESULT.json
Original raw SHA256 68f4ea4ebd57d1fa0c272cda511e5cbbbca10f901bfef6c639a5ce72fdd325df; 448 bytes. Complete parsed object, compact encoding:

```json
{"finishedAt":"2026-10-18T12:24:42.143458+00:00","name":"ordinary-optins","exitCode":0,"wallSeconds":5.595688932997291,"stdoutSha256":"102a3124ebaa4e8fd2c836bd5ba595d5aaf63a4750bb0c4ad8aafa0f2ed12e71","stderrSha256":"c0a4394d2923d070c4b25656f7c68f711770936ce4376d028029cb5085fb677d","headAfter":"ff4f718d8b1d9deccf40ff7597b6afbb1f1ccf75","treeAfter":"3b5d75a115712b155b68398d83ce18cef78a3d87","statusAfter":""}
```

## ACTUAL direct-query receipt ordinary-optins/QUERY-RESULT.json
Original raw SHA256 0bc07b7beab597764cb29ec8cd7bb96839da5ad232a148d70e7c698e935bb0ea; 398 bytes. Complete parsed object, compact encoding:

```json
{"argv":["/usr/bin/timeout","--signal=TERM","--kill-after=5s","10s","/tmp/doctorwatson-capabilities-bj33cu41/target/debug/mori","--color","never","capabilities","--json"],"exitCode":0,"stderrEmpty":true,"completeJSONEqualsSpecifiedContract":true,"stdoutSha256":"741d77f226442afa8665f107442e17e52787968a2da75df3c09778b9ded3bf00"}
```

## ACTUAL test stdout ordinary-optins/STDOUT.log
Raw SHA256 102a3124ebaa4e8fd2c836bd5ba595d5aaf63a4750bb0c4ad8aafa0f2ed12e71; 198 bytes.

```text

running 1 test
test public_query_distinguishes_profiles_build_consumers_and_unchecked_targets ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.01s


```

## ACTUAL direct-query stderr ordinary-optins/QUERY-STDERR.log
Original file is exactly0 bytes, SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855.

## ACTUAL original CHECK-PLAN.json
Original raw SHA256 fd39e9ff13499082a0044326e5927e124034fada6c20602cba81ebb8101ee59e; 5333 bytes. Complete parsed object, compact encoding:

```json
{"recordedAt":"2026-10-18T12:23:42.578594+00:00","head":"ff4f718d8b1d9deccf40ff7597b6afbb1f1ccf75","tree":"3b5d75a115712b155b68398d83ce18cef78a3d87","sourcePins":819,"toolPins":[{"path":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo","sha256":"e951141cc55a6cd7b9876d187bd30a3720e6086b99413af95e3b8de1cdd72f14"},{"path":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc","sha256":"f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f"},{"path":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc","sha256":"a1321a60578a6b2e3e0ff428201b8618bbd9a9fbcd475c0d0f151bbf17cb0a9e"},{"path":"/usr/bin/timeout","sha256":"f84e76ea4da1e0c4e92254bd0d7dccdabe2c8b3098e734f27997dae3ee5d8e02"}],"environment":{"PATH":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin","RUSTC":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc","RUSTDOC":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc","CARGO_HOME":"/tmp/doctorwatson-capabilities-bj33cu41/cargo-home","CARGO_TARGET_DIR":"/tmp/doctorwatson-capabilities-bj33cu41/target","CARGO_NET_OFFLINE":"true","CARGO_INCREMENTAL":"0","CARGO_CACHE_AUTO_CLEAN_FREQUENCY":"never","GIT_CONFIG_NOSYSTEM":"1","GIT_CONFIG_GLOBAL":"/dev/null","GIT_CONFIG_SYSTEM":"/dev/null","LANG":"C.UTF-8","TMPDIR":"/tmp/doctorwatson-capabilities-bj33cu41/tmp"},"ownedTemporaryRoot":"/tmp/doctorwatson-capabilities-bj33cu41","absentAncestorConfigs":["/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/.cargo/config","/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/.cargo/config.toml","/home/charl/Moriarty/.worktrees/.cargo/config","/home/charl/Moriarty/.worktrees/.cargo/config.toml","/home/charl/Moriarty/.cargo/config","/home/charl/Moriarty/.cargo/config.toml","/home/charl/.cargo/config","/home/charl/.cargo/config.toml","/home/.cargo/config","/home/.cargo/config.toml","/.cargo/config","/.cargo/config.toml","/tmp/doctorwatson-capabilities-bj33cu41/cwd/.cargo/config","/tmp/doctorwatson-capabilities-bj33cu41/cwd/.cargo/config.toml","/tmp/doctorwatson-capabilities-bj33cu41/.cargo/config","/tmp/doctorwatson-capabilities-bj33cu41/.cargo/config.toml","/tmp/.cargo/config","/tmp/.cargo/config.toml"],"expectedContractPins":[{"path":"/home/charl/moriarty-c2-opus55-restored-20261017/capabilities-real-consumer-check-plan-20261018/specified-expected/no-default-features.json","sha256":"673b3f0bc892be03f8bf34554b3567ac250e7ed8ef4e8cbfe0fcc09c00dcd9c0","provenance":"Author-specified complete public metadata contract, frozen before actual check. No monetary/work execution or independently derived financial claim."},{"path":"/home/charl/moriarty-c2-opus55-restored-20261017/capabilities-real-consumer-check-plan-20261018/specified-expected/ordinary-optins.json","sha256":"170daedfca48f25c325312f1257107f7952bd6bc568a1f9b6cc0ae4e6962a753","provenance":"Author-specified complete public metadata contract, frozen before actual check. No monetary/work execution or independently derived financial claim."}],"steps":[{"name":"no-default-features","argv":["/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo","test","--manifest-path","/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/Cargo.toml","--frozen","-j1","-p","mori","--no-default-features","--test","capabilities","--","--test-threads=1"],"wallSeconds":90,"killGraceSeconds":5},{"name":"ordinary-optins","argv":["/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo","test","--manifest-path","/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/Cargo.toml","--frozen","-j1","-p","mori","--no-default-features","--features","source6-expansion,core2-disjoint-draft","--test","capabilities","--","--exact","public_query_distinguishes_profiles_build_consumers_and_unchecked_targets","--test-threads=1"],"wallSeconds":90,"killGraceSeconds":5}],"actualQueryPerFeature":{"argv":["/tmp/doctorwatson-capabilities-bj33cu41/target/debug/mori","--color","never","capabilities","--json"],"wallSeconds":10,"killGraceSeconds":5},"scope":"Ordinary reporting and source-only check/inspect. No evaluator financial transaction, compiler, native SDK/VM/proof/key/wallet/ledger invocation; no prior cap reset.","sourceRead":"Root read whole4paths, actual dispatcher/check/inspect/print paths; v1/v2 defects preserved and v3 source corrections audited locally. Fresh independent review pending."}
```

## ACTUAL original RESULT.json
Original raw SHA256 afbb33934dbda90318711cef9d73f3503cd1fc28dabd05aaa2cda7a053f6e2d5; 1358 bytes. Complete parsed object, compact encoding:

```json
{"recordedAt":"2026-10-18T12:24:42.251039+00:00","head":"ff4f718d8b1d9deccf40ff7597b6afbb1f1ccf75","tree":"3b5d75a115712b155b68398d83ce18cef78a3d87","steps":[{"finishedAt":"2026-10-18T12:24:36.430688+00:00","name":"no-default-features","exitCode":0,"wallSeconds":53.79332958500163,"stdoutSha256":"97cdd4916ec4b71995e594879d056dd1c0be9fe82a80431668a2c764f4994c81","stderrSha256":"1b573a3442c57d4f7678a272dfee85e561ad4c15d62aa0ea4954a9459968ebb7","headAfter":"ff4f718d8b1d9deccf40ff7597b6afbb1f1ccf75","treeAfter":"3b5d75a115712b155b68398d83ce18cef78a3d87","statusAfter":""},{"finishedAt":"2026-10-18T12:24:42.143458+00:00","name":"ordinary-optins","exitCode":0,"wallSeconds":5.595688932997291,"stdoutSha256":"102a3124ebaa4e8fd2c836bd5ba595d5aaf63a4750bb0c4ad8aafa0f2ed12e71","stderrSha256":"c0a4394d2923d070c4b25656f7c68f711770936ce4376d028029cb5085fb677d","headAfter":"ff4f718d8b1d9deccf40ff7597b6afbb1f1ccf75","treeAfter":"3b5d75a115712b155b68398d83ce18cef78a3d87","statusAfter":""}],"allStepsCompleted":true,"allExitZero":true,"temporaryRootExistsAfter":false,"limits":"Selected normal reporting/source-only consumers; no proof/ledger/target acceptance."}
```

## ACTUAL original CLEANUP.json
Original raw SHA256 acb81ac66da87b8dab117dcfa4e11f8eee24644963c14c50ab224359d8aff390; 226 bytes. Complete parsed object, compact encoding:

```json
{"recordedAt":"2026-10-18T12:24:42.250863+00:00","ownedTemporaryRoot":"/tmp/doctorwatson-capabilities-bj33cu41","existsAfter":false,"scope":"Only owned temp root removed; publiccache symlinks not traversed."}
```

## Exact E4D curated-metadata context excerpts
All full context files are physically preserved. The embedded ranges below are the only context coverage delivered in this standalone text; reading them is not reading every omitted line or certifying those consumers. Context bytes are unchanged at the final head.

### E4D crates/mori/Cargo.toml; full SHA256 2202f89a5c40b7bbe503ebb66a406226c2e446ab46580e1c2ff5d4f870ecea1b; lines 1–30
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

```

### E4D crates/mori-core/Cargo.toml; full SHA256 93318da98ab974c2d30210bd868f98fa5c4af60390395dcbe4f65669e815ac7c; lines 7–26
```text

[features]
core2-disjoint-draft = []
# Opt-in and non-default: enables only the Midnight-native signing-frame
# compatibility test. Ordinary mori-core consumers and SDK-free test
# selection never pull these dependencies.
midnight-compat-tests = ["dep:midnight-base-crypto", "dep:midnight-serialize"]

[dependencies]
mori-lexer.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
sha2.workspace = true
k256.workspace = true
# Optional and off by default; pulled in only by the midnight-compat-tests
# feature for the native compatibility test.
midnight-base-crypto = { workspace = true, optional = true }
midnight-serialize = { workspace = true, optional = true }


```

### E4D crates/mori-source6/Cargo.toml; full SHA256 ab197468c5af4f407eb489f1546978f092698248e53a85e14c144deebe80e78e; lines 1–24
```text
[package]
name = "mori-source6"
version = "0.1.0"
description = "Local transfer presentation expansion from checked Moriarty beta source"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[features]
default = []
source6-transfer-slice = []

[dependencies]
mori-ast.workspace = true
mori-checker.workspace = true
mori-diagnostics.workspace = true
mori-lexer.workspace = true
mori-parser.workspace = true
mori-span.workspace = true
serde_json.workspace = true

[dev-dependencies]
insta.workspace = true

```

### E4D crates/mori-core/src/ir.rs; full SHA256 0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d; lines 1–8
```text
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const LANGUAGE: &str = "moriarty/1";
pub const SEMANTICS: &str = "moriarty-core/1";
pub const MAX_EXPRESSIONS: usize = 8192;
pub const MAX_STATEMENTS: usize = 1024;
pub const MAX_DECLARATIONS: usize = 256;

```

### E4D crates/mori-parser/src/lib.rs; full SHA256 3e03133719667b25827261a8d5ac1cf6995b0365057e1c9ef89c597acef67b99; lines 14–29
```text

use mori_ast::Ast;
use mori_diagnostics::MoriDiagnostic;

/// The only profile this parser accepts.
pub const PROFILE: &str = "moriarty-beta/1";
/// Deepest nesting of expressions and types.
pub const MAX_DEPTH: usize = 64;
/// Most fields in one record or call, and most type arguments.
pub const MAX_FIELDS: usize = 64;
/// Most declarations and actions in one agreement.
pub const MAX_ITEMS: usize = 256;

/// Lexes and parses `source`, or reports the first error.
pub fn parse(source: &str) -> Result<Ast<'_>, MoriDiagnostic> {
    let tokens = mori_lexer::lex(source)?;

```

### E4D crates/mori-core/src/stage/mod.rs; full SHA256 e3c4a463319056d0a32ac192c57b3f587584976e08974f4dec04c9e881f80f23; lines 1–6
```text
//! Staged `moriarty/2` programs and their local continuation interface.
//!
//! Local and unsigned only. Event binders and captures are caller-chosen values
//! (no authentication), a continuation can be resumed any number of times (no
//! consumption tracking), and no stage authority exists. Outputs carry
//! `STAGE_LABELS` so these limits stay visible.

```

E4D lines 27–42 (same file/pin):
```text
pub const LANGUAGE_2: &str = "moriarty/2";
pub const SEMANTICS_2: &str = "moriarty-core/2";
pub const STAGE_PLAN_FORMAT: &str = "moriarty-stage-plan/1";
pub const CONTINUATION_FORMAT: &str = "moriarty-continuation/1";
pub const STAGE_EVENT_FORMAT: &str = "moriarty-stage-event/1";
pub const MAX_AWAITS_PER_ACTION: usize = 16;
pub const MAX_EVENT_BINDERS: usize = 16;
pub const MAX_CAPTURE_SLOTS: usize = 256;
pub const MAX_CONTINUATION_BYTES: usize = 4 * 1_048_576;
pub const MAX_EVENT_BYTES: usize = 65_536;
/// Inserted verbatim into every `/2` output object.
pub const STAGE_LABELS: [(&str, &str); 3] = [
    ("qualification", "local-reference-stage"),
    ("authentication", "none"),
    ("consumption", "untracked"),
];

```

E4D lines 73–96 (same file/pin):
```text
/// Parses, admits and plans a `moriarty/2` source. Legacy `parse`/`validate`
/// remain `/1`-only, so a `/2` source fails every legacy consumer.
pub fn load(src: &str) -> Result<StagedProgram, Error> {
    let program = crate::parser::parse_v2(src)?;
    profile::require_core2(&program)?;
    validate::validate_v2(&program)?;
    let mut plan = plan::build(&program)?;
    validate::plan_invariants(&program, &plan)?;
    cost::fill(&program, &mut plan)?;
    validate::cost_invariants(&program, &plan)?;
    let program_hash = digest::program_hash(&program)?;
    plan.program_hash = digest::hex(&program_hash);
    let plan_hash = digest::plan_hash(&plan)?;
    Ok(StagedProgram {
        program,
        plan,
        program_hash,
        plan_hash,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamedValue {

```

### E4D crates/mori/src/commands/check.rs; full SHA256 2199d7b49173816c527436eef3cf30c2cb51d1b039603b1046e7bd508762faf7; lines 21–35
```text
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

```

E4D lines 38–47 (same file/pin):
```text
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

```

E4D lines 62–64 (same file/pin):
```text
    let (accepted, report) = match mori_parser::parse(&source) {
        Err(error) => {
            let diagnostics = vec![error];

```

E4D lines 78–83 (same file/pin):
```text
                );
            }
            let result = mori_checker::check(&ast);
            let report = args.json.then(|| {
                CheckReport::new(
                    name.clone(),

```

### E4D crates/mori/src/commands/inspect.rs; full SHA256 e17139f43eaefc07530a2f9636e0b42fe2410ce8bcef1710f206ebc1bcc4e5c8; lines 22–59
```text
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    #[cfg(not(feature = "native"))]
    if args.target.is_some() {
        return super::native_unavailable();
    }
    let source = super::core_io::read_source(&args.file)?;
    if args.target.is_none()
        && mori_core::language(&source).is_ok_and(|language| language == "moriarty/2")
    {
        let staged = match mori_core::stage::load(&source) {
            Ok(staged) => staged,
            Err(error) => {
                super::simulate::print_stage_report(json!({"status":"rejected", "error":error}))?;
                return Ok(ExitCode::FAILURE);
            }
        };
        let program = staged.program();
        super::simulate::print_stage_report(json!({
            "status": "accepted", "language": program.language, "semantics": program.semantics,
            "agreement": program.name, "states": program.states, "assets": program.assets,
            "actions": program.actions, "expressions": program.expressions,
            "stage_plan": staged.plan(), "enforcement_report": null
        }))?;
        return Ok(ExitCode::SUCCESS);
    }
    #[cfg(feature = "native")]
    if mori_core::language(&source).is_ok_and(|language| language == "moriarty/2") {
        // The enforcement map reports the /3 relation. A Core2 --target refuses
        // with Core's typed profile error before reading the target, backend
        // layout or any output; source-only Core2 inspect is handled above.
        let staged = mori_core::stage::load(&source).map_err(super::core_io::operation)?;
        let refusal = match mori_core::stage::require_core1(staged.program()) {
            Err(error) => error,
            Ok(()) => core1_refusal(),
        };
        return Err(super::core_io::operation(refusal));
    }
    let program = mori_core::parse(&source).map_err(super::core_io::operation)?;

```

E4D lines 78–106 (same file/pin):
```text
    let qualification = if enforcement.is_null() {
        "source-only-unqualified"
    } else {
        "local-compiled-unqualified"
    };
    // Export validates depth/work/byte bounds and escapes public labels before
    // printing anything. A compiler or report-size refusal leaves stdout empty.
    super::core_io::print(&json!({
        "format": "moriarty-enforcement-report/1",
        "qualification": qualification,
        "language": program.language, "semantics": program.semantics,
        "agreement": program.name, "states": program.states,
        "assets": program.assets, "actions": program.actions,
        "expressions": program.expressions,
        "enforcement_report": enforcement,
        "open_premises": [
            "a supplied exact target is required for the compiled enforcement map",
            "ledger authentication and acceptance",
            "native proof soundness and compiler correspondence",
            "account and sponsor authority"
        ]
    }))?;
    Ok(ExitCode::SUCCESS)
}

#[cfg(feature = "native")]
fn encoded(predicates: &[&str]) -> Value {
    json!({"status": "encoded-not-verified", "predicates": predicates})
}

```

### E4D crates/mori/src/commands/simulate.rs; full SHA256 53b1c0cb0bbaa94cf890890c37e1a282535e5e8bbcf29dafaaef498faed9ac11; lines 104–116
```text
    if args.target.is_some() || args.authorization.present() {
        return Err(super::core_io::operation(
            "--target and signed replay arguments require --trace",
        ));
    }
    let source = super::core_io::read_source(&args.file)?;
    // Hook: /2 routing. A /2 start keeps every core failure labelled.
    if mori_core::language(&source) == Ok(mori_core::stage::LANGUAGE_2) {
        return run_stage_start(&args, &source, action, scenario_path);
    }
    let program = mori_core::parse(&source).map_err(super::core_io::operation)?;
    let scenario = super::core_io::read_json(scenario_path)?;
    if let Some(cap) = args.logical_cap {

```

E4D lines 195–206 (same file/pin):
```text
                .and_then(|text| decode::event(&text))
                .map_err(no_work)?;
        stage::resume(
            &program,
            continuation,
            event,
            args.action.as_deref(),
            args.logical_cap,
        )
    })();
    print_stage_result(result)
}

```

E4D lines 228–232 (same file/pin):
```text
            })
        })?;
        stage::start(&program, action, scenario, args.logical_cap)
    })();
    print_stage_result(result)

```

### E4D crates/mori-core/src/stage/report.rs; full SHA256 25e2b88e527a3f9b90951a92fc92ba8c17dd6f16040529dcac0356bb36707b4f; lines 1–38
```text
//! Complete qualified JSON envelopes for the local Core/2 outcome API.
use super::{STAGE_LABELS, StageOutcome, StageRejection};
use serde_json::{Value, json};

/// Render an outcome or rejection without executing a program or performing I/O.
/// Every envelope retains the explicit local-stage qualifications.
pub fn outcome_report(result: Result<StageOutcome, StageRejection>) -> Value {
    let mut report = match result {
        Ok(StageOutcome::Waiting {
            advanced,
            segment,
            continuation,
            attempted,
            residual,
        }) => {
            let committed = continuation.logical.committed;
            json!({"status":"waiting","advanced":advanced,"segment":segment,
                "continuation":continuation,"attempted":attempted,
                "committed":committed,"residual":residual})
        }
        Ok(StageOutcome::Complete {
            segment,
            committed,
            attempted,
            residual,
        }) => json!({"status":"complete","segment":segment,"attempted":attempted,
                "committed":committed,"residual":residual}),
        Err(rejected) => json!({"status":"rejected","error":rejected.error,
            "attempted":rejected.attempted,"committed":rejected.committed}),
    };
    let object = report
        .as_object_mut()
        .expect("stage reports are JSON objects");
    for (name, value) in STAGE_LABELS {
        object.insert(name.into(), Value::String(value.into()));
    }
    report
}

```

### E4D crates/mori/src/commands/test.rs; full SHA256 6d79546f1bf9c6271ff45ca4e58b514cc2fc6f5a6b970b55f6b3df4e6788aec8; lines 36–56
```text
    Rejected {
        code: String,
    },
}

pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let value: serde_json::Value = core_io::read_json(&args.dir.join("mori-tests.json"))?;
    if value.get("profile").and_then(serde_json::Value::as_str) == Some("moriarty-tests/2") {
        return stage_test::run(&args.dir, value, shell);
    }
    if value.get("profile").and_then(serde_json::Value::as_str)
        == Some("moriarty-tests/disjoint-draft/1")
    {
        #[cfg(feature = "core2-disjoint-draft")]
        return disjoint_test::run(&args.dir, value, shell);
        #[cfg(not(feature = "core2-disjoint-draft"))]
        return Err(core_io::operation(
            "disjoint draft requires feature core2-disjoint-draft",
        ));
    }
    let manifest: Manifest = serde_json::from_value(value).map_err(core_io::operation)?;

```

### E4D crates/mori/src/commands/expand.rs; full SHA256 f7ff48dcaece95b05392e25ad3a9dff66dd736f839ceaa2928bf9d768aedc320; lines 5–8
```text

#[cfg(feature = "source6-expansion")]
mod source6;


```

E4D lines 21–45 (same file/pin):
```text

pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    if args.emit_source6_transfer {
        return run_source6(args.input, shell);
    }
    let (_, program) = super::core_io::program(&args.input.file)?;
    // Validate the selected scenario/action without discarding the generic core.
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    mori_core::simulate(&program, &args.input.action, &scenario)
        .map_err(super::core_io::operation)?;
    super::core_io::print(&program)?;
    Ok(ExitCode::SUCCESS)
}

#[cfg(feature = "source6-expansion")]
fn run_source6(input: ActionInput, shell: &mut Shell) -> super::Result<ExitCode> {
    source6::run(input, shell)
}

#[cfg(not(feature = "source6-expansion"))]
fn run_source6(_input: ActionInput, _shell: &mut Shell) -> super::Result<ExitCode> {
    Err(crate::error::CliError::Operation {
        code: "mori::command::source6-unavailable".into(),
        message: "Source/6 transfer expansion requires the source6-expansion build feature.".into(),
    }

```

### E4D crates/mori/src/commands/expand/source6.rs; full SHA256 e38a9659b993ea15da96931ba1fed0f34cab796b8e8efaeb3a6f8e27a5e4aca7; lines 90–105
```text
    let raw: serde_json::Value = core_io::read_json(path)?;
    let object = raw.as_object().ok_or_else(|| {
        scenario_error(
            "source6-scenario-profile",
            "Expected a local transfer scenario object.",
        )
    })?;
    if object.get("profile").and_then(serde_json::Value::as_str)
        != Some("moriarty-local-scenario/1")
        || object.get("kind").and_then(serde_json::Value::as_str) != Some("local-stipulation")
    {
        return Err(scenario_error(
            "source6-scenario-profile",
            "Source/6 transfer expansion requires moriarty-local-scenario/1 local-stipulation input.",
        ));
    }

```

E4D lines 133–141 (same file/pin):
```text
    let source = core_io::read_source(&input.file)?;
    let name = crate::display::text(&input.file.display().to_string());
    let selected = match select_transfer(&source, &input.action) {
        Ok(selected) => selected,
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                shell.report(&diagnostic.with_source_code(NamedSource::new(&name, source.clone())));
            }
            return Ok(ExitCode::FAILURE);

```

### E4D crates/mori-source6/src/lib.rs; full SHA256 f89e8b91241742a34ec20545411bfa24f1c4833685ce0fd39290dec3a995af4a; lines 1–6
```text
//! Local Source/6 presentation for the closed beta transfer slice.
//!
//! This crate emits a proposal. Its scenario cells and declared hashes remain
//! claims; it performs no Core/5 preparation, signing, proof or ledger action.
//! The generated text must be consumed by the existing Source/6 lowerer before
//! any claim of Source/6 formation or Core/5 preparation.

```

### E4D crates/mori-source6/src/transfer.rs; full SHA256 60b76bb0c4ff4b588db54793b32cbf5b108af2e98e340b8b60de46a6c1144c08; lines 81–94
```text
/// Parse, check and select a beta transfer before a caller reads its scenario.
/// Existing checker diagnostics and unsupported-action selection are retained.
pub fn select_transfer(
    source: &str,
    action: &str,
) -> std::result::Result<SelectedTransfer, Vec<MoriDiagnostic>> {
    let ast = mori_parser::parse(source).map_err(|error| vec![error])?;
    let result = mori_checker::check(&ast);
    if !result.is_accepted() {
        return Err(result.diagnostics);
    }
    let (terms, span) = extract(&result.checked, action).map_err(|error| vec![error])?;
    Ok(SelectedTransfer { terms, span })
}

```

E4D lines 190–202 (same file/pin):
```text
    let index = index.ok_or_else(|| diagnostics::action(ast.span(NodeIdx::ROOT), action))?;
    let node = checked.actions.node()[index];
    let span = ast.span(node);
    if checked.actions.support()[index] != Some(Support::LocalS0) {
        return Err(diagnostics::unsupported(span));
    }
    let intent = checked.actions.intent()[index]
        .and_then(|decl| checked.value(decl))
        .ok_or_else(|| diagnostics::shape(span, "a resolved checked intent"))?;
    let operation = field(checked, intent, "operation")?;
    if call_name(checked, operation) != "transfer" {
        return Err(diagnostics::unsupported(span));
    }

```

### E4D crates/mori/src/commands/compile.rs; full SHA256 897a45c35c2fc8c3371e02d18b0160f224861b53d3ececcd0381a7a6110a3708; lines 20–29
```text
#[cfg(feature = "native")]
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let source = core_io::read_source(&args.file)?;
    if mori_core::language(&source).is_ok_and(|language| language == "moriarty/2") {
        let target: mori_backend::Target = core_io::read_json(&args.target)?;
        let artifact = mori_backend::stage::compile_staged(&source, &target, &args.out)
            .map_err(core_io::operation)?;
        let manifest = serde_json::to_string_pretty(&artifact).map_err(core_io::operation)?;
        std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
            path: args.out.clone(),

```

E4D lines 43–49 (same file/pin):
```text
    let program = mori_core::parse(&source).map_err(core_io::operation)?;
    let target: mori_backend::Target = core_io::read_json(&args.target)?;
    // The trusted Target selects the mode explicitly; a signed Target never
    // reaches the unsigned compiler.
    let reported = mori_backend::source_projection::compile_reported(&program, &target)
        .map_err(core_io::operation)?;
    let artifact = reported.artifact;

```

E4D lines 267–275 (same file/pin):
```text
#[cfg(not(feature = "native"))]
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let _ = (
        &args.file,
        &args.target,
        &args.out,
    );
    super::native_unavailable()
}

```

### E4D crates/mori-backend/src/lib.rs; full SHA256 3ad0862058adbdc9b9a080f644fec7aa1721d70f115cdf8a08f5ef45cbdf2259; lines 30–37
```text
pub mod proving;

pub const ARTIFACT_FORMAT: &str = "moriarty-zkir-artifact/3";
pub const PUBLIC_ABI_VERSION: &str = "moriarty-public-abi/3";
pub const RELATION_VERSION: &str = "moriarty-zkir-relation/3";
/// Explicit authorization discriminator of an unsigned v3 artifact.
pub const UNSIGNED_MODE: &str = "unsigned";


```

### E4D crates/mori-backend/src/stage/mod.rs; full SHA256 69fdceb125c3ef29a29f80b45b54f09eb601311d3ed181a2579f85049b796693; lines 1–6
```text
//! Staged `moriarty/2` compilation to `/4` artifacts: N + 1 successful-segment
//! circuits per action with N awaits, through the ordinary `/3` lowerer.
//! `/4` is a distinct type with no conversion to the `/3` Artifact, so proving,
//! signed and native consumers cannot accept it. Unsigned only: there is no
//! event authentication, stage authority or consumption, and all ten gates
//! remain unperformed.

```

### E4D crates/mori-backend/src/stage/artifact.rs; full SHA256 cf792142ec2ed0eec84b5fec5dbaa81eecdac18f3ac11cae0a8c08ca9759db72; lines 19–24
```text
pub const STAGED_ARTIFACT_FORMAT: &str = "moriarty-zkir-artifact/4";
pub const STAGED_ABI_VERSION: &str = "moriarty-public-abi/4";
pub const STAGED_RELATION_VERSION: &str = "moriarty-zkir-relation/4";
pub const STAGE_CONTEXT_FORMAT: &str = "moriarty-stage-context/1";
/// Logical work is static plan metadata: not circuit-enforced and not DUST.
pub const LOGICAL_ENFORCEMENT: &str = "not-circuit-enforced-not-dust";

```

### E4D crates/mori/src/commands/lsp.rs; full SHA256 8187ef893f48b6a5a54a5d48e61e120edde48ddb57b64d21b46100970d8df478; lines 1–10
```text
use std::process::ExitCode;

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {}

pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    todo!("mori lsp")
}

```

### E4D crates/mori/src/commands/mcp.rs; full SHA256 778c04c0168fd45efc8823a4f329f6b332f8808045b58d4fbccb58f279119756; lines 1–10
```text
use std::process::ExitCode;

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {}

pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    todo!("mori mcp")
}

```

### E4D crates/mori/src/commands/core_io.rs; full SHA256 6ebccb1465788264448ac791dea248e68a1cea71942496ee303ead619672b594; lines 247–251
```text
pub fn print<T: Serialize>(value: &T) -> super::Result<()> {
    let display = mori_core::json::export(value).map_err(operation)?;
    println!("{display}");
    Ok(())
}

```

### E4D packages/moriarty-beta/package.json; full SHA256 d7b1f8f111fb081f18a80a965019d8a95795fe086df626992e62674fbb234338; lines 1–12
```text
{
  "name": "@moriarty-lang/beta",
  "version": "0.1.0-beta.1",
  "type": "module",
  "description": "Bounded Moriarty authoring and unqualified local S0 preparation",
  "engines": {
    "node": ">=24"
  },
  "bin": {
    "mori": "dist/cli.js"
  },
  "exports": {

```

## Actual-result interpretation and unread scope

Observed: no-default2 tests passed; opt-ins1 passed/1 filtered. Wrapper53.7933s/5.5957s include builds, not query latency. Only direct-query stderr is empty. Cargo progress stderr is preserved physically, not embedded. Both direct queries exit0 with complete parsed equality to the frozen author metadata contracts; this is ordinary report/client evidence, not compiler/proof/financial execution.

Receipts record unchanged head/tree, clean status and deleted owned temp root. Transport rechecked source/log/query hashes and equality; this is custody, not approval. Full changed4 files and actual/expected JSON are embedded. Unembedded: original819-entry pin manifest, Cargo stderr and context outside stated excerpt ranges (all28 full files preserved physically). Do not assert these were read or invent tool/model attestation. No provider private stream is supplied. Cite actual delivered evidence and retain these limits; root decides adoption.
