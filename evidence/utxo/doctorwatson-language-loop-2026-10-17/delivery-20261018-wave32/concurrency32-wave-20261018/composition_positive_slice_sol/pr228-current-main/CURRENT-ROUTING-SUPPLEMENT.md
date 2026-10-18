
## Complete current main routing source crates/mori/src/cli.rs
```text
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

## Complete current main routing source crates/mori/src/commands/mod.rs
```text
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

## Complete current main routing source crates/mori/src/commands/capabilities.rs
```text
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
