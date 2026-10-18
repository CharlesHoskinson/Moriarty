# Finite changed-base integration confirmation DRAFT
Both original fresh source/resulta1 contexts approved the exact Source6 source8267ad950742c94f74fd871c266fa995c4e9e893/treea936ca2a3383045b38e90cc3ab867ae948fa64e1 and captured4commands/12tests, BUT each explicitly conditioned merge on unchanged base e3eb6d12aba0c301d32b278bf9bdd6f98155308f. That guard is unsatisfied after root-reported main advance; the original WHOLE objects remain untouched. This task requests a NEW narrowly finite integration judgment over exact pinned intervening changes, NOT transferred old source approval or a blanket permission for future unrelated changes. Actual final-base root facts/expectedmerge must be present before dispatch. The full original39source/context/lock/result/oracle/postcapturecomparison/review packet is physically nested; do not repeat its109KB primary or regenerate any product output.

## COMPLETE unchanged selected dispatch/parser/consumer/test crates/mori/src/main.rs SHA256 35988592bec7038a7674403a0e897184d1099dd4164e89a3ca0aa22ab0b3a232
```
use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;
use crate::shell::Shell;

mod cli;
mod commands;
mod display;
mod error;
mod json;
mod shell;
mod styles;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut shell = Shell::new(cli.global.verbosity(), cli.global.color.into());

    match commands::run(cli.command, &mut shell) {
        Ok(code) => code,
        Err(report) => {
            shell.report(&report);
            ExitCode::FAILURE
        }
    }
}

```

## COMPLETE unchanged selected dispatch/parser/consumer/test crates/mori/src/cli.rs SHA256 3c16445a29ec536588efd1613d0782834e36dbdcfc11d874b8fa0f4a1d4033d2
```
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::commands::{
    check, compile, compile_call, expand, format, init, inspect, intent, keygen_call,
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

## COMPLETE unchanged selected dispatch/parser/consumer/test crates/mori/src/commands/mod.rs SHA256 46a297c1a63b440a3fff1fddacac3953c36e39cc3e3b088ba229494005e8d091
```
//! One module per subcommand. Each defines its `Args` and a `run` function.

use std::process::ExitCode;

use mori_diagnostics::Report;

use crate::cli::Command;
use crate::shell::Shell;

mod call_io;
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

## COMPLETE unchanged selected dispatch/parser/consumer/test crates/mori/src/commands/expand.rs SHA256 f7ff48dcaece95b05392e25ad3a9dff66dd736f839ceaa2928bf9d768aedc320
```
use std::process::ExitCode;

use crate::cli::ActionInput;
use crate::shell::Shell;

#[cfg(feature = "source6-expansion")]
mod source6;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Emit a local Source/6 transfer proposal; requires source6-expansion
    #[arg(
        long,
        long_help = "Emit raw local Source/6 transfer proposal text. Requires the source6-expansion build feature; default Core expansion is unchanged."
    )]
    pub emit_source6_transfer: bool,
}

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
    .into())
}

```

## COMPLETE unchanged selected dispatch/parser/consumer/test crates/mori/src/commands/expand/source6.rs SHA256 e38a9659b993ea15da96931ba1fed0f34cab796b8e8efaeb3a6f8e27a5e4aca7
```
//! Explicit local transfer presentation consumer of mori-source6.
use std::{io::Write, process::ExitCode};

use mori_core::UInt;
use mori_diagnostics::NamedSource;
use mori_source6::{Allowance, Balance, LocalTransferScenario, Replay, select_transfer};
use serde::Deserialize;

use crate::{cli::ActionInput, commands::core_io, shell::Shell};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TransferScenario {
    #[serde(rename = "profile")]
    _profile: String,
    #[serde(rename = "kind")]
    _kind: String,
    domain: String,
    asset: String,
    head: String,
    predecessor: String,
    round: UInt,
    balances: [BalanceCell; 3],
    allowance: AllowanceCell,
    replay: ReplayClaim,
    work_remaining: UInt,
    work_spent: UInt,
    post_head: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BalanceCell {
    account: String,
    amount: UInt,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AllowanceCell {
    owner: String,
    remaining: UInt,
    spent: UInt,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ReplayClaim {
    Unused,
    Consumed,
}

impl TransferScenario {
    fn local(&self) -> LocalTransferScenario<'_> {
        LocalTransferScenario {
            domain: &self.domain,
            asset: &self.asset,
            head: &self.head,
            predecessor: &self.predecessor,
            round: self.round.0,
            balances: self.balances.each_ref().map(|cell| Balance {
                account: &cell.account,
                amount: cell.amount.0,
            }),
            allowance: Allowance {
                owner: &self.allowance.owner,
                remaining: self.allowance.remaining.0,
                spent: self.allowance.spent.0,
            },
            replay: match self.replay {
                ReplayClaim::Unused => Replay::Unused,
                ReplayClaim::Consumed => Replay::Consumed,
            },
            work_remaining: self.work_remaining.0,
            work_spent: self.work_spent.0,
            post_head: &self.post_head,
        }
    }
}

fn scenario_error(code: &str, message: impl Into<String>) -> mori_diagnostics::Report {
    crate::error::CliError::Operation {
        code: format!("mori::command::{code}"),
        message: crate::display::text(&message.into()),
    }
    .into()
}

fn read_scenario(path: &std::path::Path) -> crate::commands::Result<TransferScenario> {
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
    for field in ["obligation", "candidate_effects"] {
        if object.contains_key(field) {
            return Err(scenario_error(
                "source6-scenario-unsupported",
                format!("The --emit-source6-transfer subset does not support `{field}`."),
            ));
        }
    }
    if object
        .get("balances")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|balances| balances.len() != 3)
    {
        return Err(scenario_error(
            "source6-scenario-cells",
            "The transfer subset requires exactly three balance cells: owner, recipient, fee recipient.",
        ));
    }
    serde_json::from_value(raw).map_err(|error| {
        scenario_error(
            "source6-scenario-shape",
            format!("Local transfer scenario shape: {error}"),
        )
    })
}

pub(super) fn run(input: ActionInput, shell: &mut Shell) -> crate::commands::Result<ExitCode> {
    let source = core_io::read_source(&input.file)?;
    let name = crate::display::text(&input.file.display().to_string());
    let selected = match select_transfer(&source, &input.action) {
        Ok(selected) => selected,
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                shell.report(&diagnostic.with_source_code(NamedSource::new(&name, source.clone())));
            }
            return Ok(ExitCode::FAILURE);
        }
    };
    let scenario = read_scenario(&input.scenario)?;
    let expansion = match selected.expand(&scenario.local()) {
        Ok(expansion) => expansion,
        Err(diagnostic) => {
            shell.report(&diagnostic.with_source_code(NamedSource::new(name, source)));
            return Ok(ExitCode::FAILURE);
        }
    };
    // The library includes its final newline. This writes its actual proposal.
    std::io::stdout()
        .lock()
        .write_all(expansion.source6.as_bytes())
        .map_err(core_io::operation)?;
    shell.status(
        "Expanded",
        format!("Source/6 transfer proposal ({})", expansion.qualification),
    );
    Ok(ExitCode::SUCCESS)
}

```

## Complete source-exact selected CLI test invocation/argument helpers (all7test bodies retained physically)
```rust
#![cfg(feature = "source6-expansion")]

use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

macro_rules! fixture {
    ($name:literal) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/source6-expansion/",
            $name
        ))
        .as_slice()
    };
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Files(PathBuf);

impl Files {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-source6-expand-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir(&root).unwrap();
        fs::write(root.join("invoice.mori"), fixture!("invoice.mori")).unwrap();
        fs::write(
            root.join("scenario.json"),
            fixture!("invoice.scenario.json"),
        )
        .unwrap();
        fs::write(root.join("core.mori"), fixture!("core.mori")).unwrap();
        fs::write(root.join("core.json"), fixture!("core.scenario.json")).unwrap();
        fs::write(root.join("repayment.mori"), fixture!("repayment.mori")).unwrap();
        fs::write(root.join("oracle.mori"), fixture!("oracle.mori")).unwrap();
        Self(root)
    }

    fn invoke(&self, arguments: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.0)
            .args(["--color", "never"])
            .args(arguments)
            .output()
            .unwrap()
    }
}

impl Drop for Files {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn source6_arguments() -> [&'static str; 7] {
    [
        "expand",
        "invoice.mori",
        "--action",
        "pay",
        "--scenario",
        "scenario.json",
        "--emit-source6-transfer",
    ]
}

```

## COMPLETE unchanged selected dispatch/parser/consumer/test crates/mori/Cargo.toml SHA256 317bf14a6f3b4b26f223ac9dc93823e7d185333a161bd3ad3f0cf1e0fb6dd05f
```
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

## COMPLETE unchanged selected dispatch/parser/consumer/test crates/mori-source6/Cargo.toml SHA256 ab197468c5af4f407eb489f1546978f092698248e53a85e14c144deebe80e78e
```
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

## COMPLETE unchanged selected dispatch/parser/consumer/test experiments/moriarty-language/regressions/rust-source6-transfer.mjs SHA256 ea420fd5230700103762acfb8de3270bf46a1f6b49a6257cefab10d1ba39fc27
```
// Opt-in public Rust CLI -> existing experimental Source/6/Core/5 consumer.
// This is a local, provisional, unqualified comparison; never ledger admission.
import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { isAbsolute, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseAndLowerSource6 } from '../src/successor/financial-agreement-source-v6-frontend.ts';
import { prepareSource6S0Unqualified } from '../src/successor/mil4-s0-source-v6.ts';

const fixtures = fileURLToPath(new URL('./fixtures/source6-transfer/', import.meta.url));
const binary = process.env.MORI_RUST_BIN;
const observations = process.env.MORI_SOURCE6_OBSERVATIONS;
assert.ok(binary && isAbsolute(binary), 'MORI_RUST_BIN must name the absolute reviewed Rust binary');
assert.ok(observations && isAbsolute(observations), 'MORI_SOURCE6_OBSERVATIONS must name a fresh absolute directory');
// Refuse an existing destination so a run cannot overwrite earlier observations.
mkdirSync(observations);
const json = (name) => JSON.parse(readFileSync(join(fixtures, name), 'utf8'));
const pins = json('FIXTURE-PINS.json');
for (const [name, expected] of Object.entries(pins.files)) {
  const actual = createHash('sha256').update(readFileSync(join(fixtures, name))).digest('hex');
  assert.equal(actual, expected, `custodian fixture bytes changed: ${name}`);
}
const facts = json('FINANCIAL-PRE-POST.json');
writeFileSync(join(observations, 'run.json'), JSON.stringify({
  node: process.version, binary,
  binarySha256: createHash('sha256').update(readFileSync(binary)).digest('hex'),
  financialFreezeSha256: pins.custodianFreezeSha256,
  qualification: 'local-stipulation-only; provisional S0; no runtime qualification beyond recorded checks',
  unasserted: ['whole emitted wire golden', 'success work policy/cost/post counters',
    'authentication', 'native embedding', 'proof', 'ledger', 'parent acceptance'],
}, null, 2) + '\n');

// Only a field-name adapter into the independent constructor/arguments schema.
// No quantities, effects or post-state are calculated for the product here.
function semanticEffects(effects, asset) {
  return effects.map((effect) => {
    let args;
    switch (effect.kind) {
      case 'Debit':
      case 'Credit':
        assert.equal(effect.asset, asset);
        args = [effect.account, effect.amount];
        break;
      case 'UseAllowance': args = [effect.owner, effect.amount]; break;
      // The public Source/6 contract defines the Core replay tuple encoding.
      case 'UseReplay': args = [JSON.parse(effect.key)]; break;
      case 'AdvanceHead': args = [effect.predecessor, effect.successor]; break;
      default: assert.fail(`unexpected effect in transfer-only case: ${effect.kind}`);
    }
    return { constructor: effect.kind, arguments: args };
  });
}

for (const expected of facts.cases) {
  test(`${expected.name}: public Rust output through provisional real Source/6/Core/5 consumer`, () => {
    const scenario = json(expected.scenario);
    const quantities = expected.exact_quantities;
    const pre = expected.financial_pre;
    const post = expected.financial_post_if_local_S0_transfer_admitted;
    const args = ['--quiet', '--color', 'never', 'expand', join(fixtures, expected.source),
      '--action', expected.action, '--scenario', join(fixtures, expected.scenario),
      '--emit-source6-transfer'];
    const output = spawnSync(binary, args, {
      cwd: fixtures, stdio: ['ignore', 'pipe', 'pipe'], shell: false,
      timeout: 30_000, maxBuffer: 1024 * 1024,
    });
    // Capture bytes before any result assertion. Never substitute fixture Source/6.
    const destination = join(observations, expected.name);
    mkdirSync(destination);
    writeFileSync(join(destination, 'stdout.source6'), output.stdout ?? Buffer.alloc(0));
    writeFileSync(join(destination, 'stderr.txt'), output.stderr ?? Buffer.alloc(0));
    writeFileSync(join(destination, 'process.json'), JSON.stringify({
      executable: binary, args, cwd: fixtures, status: output.status, signal: output.signal,
      error: output.error ? { code: output.error.code, message: output.error.message } : null,
    }, null, 2) + '\n');
    assert.ifError(output.error);
    assert.equal(output.signal, null);
    assert.equal(output.status, 0, output.stderr.toString('utf8'));
    assert.equal(output.stderr.length, 0, '--quiet must suppress the status line');
    assert.ok(output.stdout.length > 0);
    assert.equal(output.stdout.at(-1), 10, 'adopted public output includes its final LF');
    assert.notEqual(output.stdout.at(-2), 10, 'adopted public output has a single final LF');
    const source6 = new TextDecoder('utf-8', { fatal: true }).decode(output.stdout);
    // No trim, JSON unwrap, AST fabrication, Node beta expansion or regeneration.
    const lowered = parseAndLowerSource6(source6);
    const outcome = prepareSource6S0Unqualified(source6);
    writeFileSync(join(destination, 'actual-consumer.json'), JSON.stringify({
      lowered, outcome,
    }, null, 2) + '\n');

    const { ast, state, intent } = lowered;
    assert.equal(ast.domain, pre.domain);
    assert.equal(ast.settlement.asset, pre.asset);
    assert.equal(ast.authenticated.head, pre.head);
    assert.equal(ast.authenticated.predecessor, scenario.predecessor);
    assert.equal(ast.authenticated.round, scenario.round);
    assert.deepEqual(ast.authenticated.balances, pre.balances);
    assert.deepEqual(ast.authenticated.allowance, pre.allowance);
    assert.equal(ast.authenticated.replay, pre.replay.status);
    assert.equal(ast.authenticated.workRemaining, expected.work.opening_remaining);
    assert.equal(ast.authenticated.workSpent, expected.work.opening_spent);
    assert.equal(state.domain, pre.domain);
    assert.equal(state.asset, pre.asset);
    assert.equal(state.head, pre.head);
    assert.equal(state.round, scenario.round);
    assert.deepEqual(state.balances, pre.balances);
    assert.deepEqual(state.allowances, [pre.allowance]);
    assert.deepEqual(state.obligations, []);
    assert.deepEqual(state.consumedReplay, []);
    assert.equal(state.workRemaining, expected.work.opening_remaining);
    assert.equal(state.workSpent, expected.work.opening_spent);
    for (const action of [ast.intent.signedAction, ast.submitted.action]) {
      assert.equal(action.kind, 'Transfer');
      assert.equal(action.value, quantities.operation_value_atomic);
      assert.equal(action.fee, quantities.operation_fee_atomic);
    }
    assert.equal(intent.kind, 'Transfer');
    assert.equal(intent.amount, quantities.operation_value_atomic);
    assert.equal(intent.fee, quantities.operation_fee_atomic);
    assert.equal(intent.grossCap, quantities.gross_cap_atomic);
    assert.equal(intent.feeCap, quantities.fee_cap_atomic);
    assert.equal(intent.netFloor, quantities.net_floor_atomic);
    assert.deepEqual(semanticEffects(lowered.submittedEffects, pre.asset), expected.ordered_semantic_effects);
    assert.equal(lowered.proposedPostHead, post.head);

    // This classification is the provisional local consumer's callable predicate,
    // not an independently frozen accepted flag or real admission assertion.
    assert.equal(outcome.status, 'PreparedUnqualified', JSON.stringify(outcome));
    const candidate = outcome.candidate;
    assert.equal(candidate.status, 'PreparedUnqualified');
    assert.deepEqual(semanticEffects(candidate.effects, pre.asset), expected.ordered_semantic_effects);
    assert.equal(candidate.preHead, pre.head);
    assert.equal(candidate.candidatePost.domain, post.domain);
    assert.equal(candidate.candidatePost.asset, post.asset);
    assert.equal(candidate.candidatePost.head, post.head);
    assert.deepEqual(candidate.candidatePost.balances, post.balances);
    assert.deepEqual(candidate.candidatePost.allowances, [post.allowance]);
    assert.deepEqual(candidate.candidatePost.obligations, []);
    assert.deepEqual(candidate.candidatePost.consumedReplay.map((key) => JSON.parse(key)),
      [post.replay.key_components]);
    // Work post counters/cost are observed in actual-consumer.json, never asserted.
    // Residual duties have no independent DTO field in this S0State projection.
  });
}

```

## Source-exact complete selected library parse/expand public functions transfer.rs lines66..132 SHA256(fullfile) 60b76bb0c4ff4b588db54793b32cbf5b108af2e98e340b8b60de46a6c1144c08
```rust
impl SelectedTransfer {
    /// Shape explicit local cells and emit a proposal, without Core preparation.
    pub fn expand(
        &self,
        scenario: &LocalTransferScenario<'_>,
    ) -> std::result::Result<TransferExpansion, MoriDiagnostic> {
        validate_scenario(&self.terms, scenario, self.span)?;
        let source6 = emit(&self.terms, scenario, self.span)?;
        Ok(TransferExpansion {
            source6,
            qualification: "local-stipulation-only",
        })
    }
}

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

/// Parse and check beta source, then emit its selected transfer proposal.
/// Quantities and claims come from the unchanged checked transfer mapping.
/// Call select_transfer first when scenario I/O must follow source selection.
pub fn expand_transfer(
    source: &str,
    action: &str,
    scenario: &LocalTransferScenario<'_>,
) -> std::result::Result<TransferExpansion, Vec<MoriDiagnostic>> {
    select_transfer(source, action)?
        .expand(scenario)
        .map_err(|error| vec![error])
}

struct Terms {
    agreement: String,
    domain: String,
    asset: String,
    scale: u128,
    signer: String,
    owner: String,
    recipient: String,
    fee_recipient: String,
    key: String,
    nonce: String,
    pre_head: String,
    valid_from: u128,
    valid_to: u128,
    gross_cap: u128,
    fee_cap: u128,
    net_floor: u128,
    value: u128,
    fee: u128,
    gross: u128,
    source_hash: String,
    policy_digest: String,
}

```

## COMPLETE exact known DevEx init module (only run/project bodies changed) SHA256 e76405597546b42227030c8a70ec67ad2f09cad9e85e97f9b6a5b81ac0eb4742
```
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::ValueEnum;

use crate::error::CliError;
use crate::shell::Shell;
use serde_json::json;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Directory to create; must not exist
    pub dir: PathBuf,

    /// Starter agreement to generate
    #[arg(long, default_value = "transfer")]
    pub template: Template,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Template {
    Transfer,
    Repay,
}

const TRANSFER: &str = r#"language "moriarty/1";
agreement StarterTransfer {
  asset TEST = { network: "local", issuer: "mori-init-example", reference: "TEST", scale: 0 };
  action pay(amount: UInt) {
    requires amount > 0;
    transfer(TEST, "alice", "bob", amount);
  }
}
"#;
const REPAY: &str = r#"language "moriarty/1";
agreement StarterRepay {
  asset TEST = { network: "local", issuer: "mori-init-example", reference: "TEST", scale: 0 };
  action seed(amount: UInt) { liability("debt", TEST, "alice", "bob", amount); }
  action repay(amount: UInt) {
    requires amount > 0;
    transfer(TEST, "alice", "bob", amount);
    discharge("debt", amount);
  }
}
"#;

/// Prepare all bytes before reserving the destination. Expectations are literal
/// financial examples, never outputs obtained from running the interpreter.
fn project(template: Template, executable: &str) -> super::Result<Vec<(&'static str, Vec<u8>)>> {
    use super::core_io::operation;
    let (source, action, amount, alice, bob) = match template {
        Template::Transfer => (TRANSFER, "pay", "25", "75", "25"),
        Template::Repay => (REPAY, "repay", "30", "70", "30"),
    };
    mori_core::parse(source).map_err(operation)?;
    let asset =
        json!({"network":"local","issuer":"mori-init-example","reference":"TEST","scale":0});
    let mut pre_liabilities = vec![];
    let mut post_liabilities = vec![];
    let mut effects =
        vec![json!({"kind":"Transfer","asset":asset,"from":"alice","to":"bob","amount":amount})];
    if matches!(template, Template::Repay) {
        pre_liabilities.push(
            json!({"name":"debt","asset":asset,"debtor":"alice","creditor":"bob","amount":"100"}),
        );
        post_liabilities.push(
            json!({"name":"debt","asset":asset,"debtor":"alice","creditor":"bob","amount":"70"}),
        );
        effects.push(json!({"kind":"Liability","name":"debt","asset":asset,"debtor":"alice","creditor":"bob","before":"100","after":"70"}));
    }
    let scenario: mori_core::Scenario = serde_json::from_value(json!({
        "state":{}, "inputs":{"amount":{"type":"UInt","value":amount}},
        "balances":[{"account":"alice","asset":asset,"amount":"100"},{"account":"bob","asset":asset,"amount":"0"}],
        "liabilities":pre_liabilities
    })).map_err(operation)?;
    let post: mori_core::Snapshot = serde_json::from_value(json!({
        "state":{},
        "balances":[{"account":"alice","asset":asset,"amount":alice},{"account":"bob","asset":asset,"amount":bob}],
        "liabilities":post_liabilities
    })).map_err(operation)?;
    let effects: Vec<mori_core::Effect> =
        serde_json::from_value(json!(effects)).map_err(operation)?;
    let manifest = json!({
        "profile":"moriarty-tests/1", "cases":[{
            "name":action, "source":"source.mori", "scenario":"scenario.json", "action":action,
            "expected":{"status":"accepted", "post":post, "effects":effects}
        }]
    });
    let scenario = mori_core::json::export(&scenario).map_err(operation)?;
    let manifest = mori_core::json::export(&manifest).map_err(operation)?;
    let readme = format!(
        "# Moriarty starter\n\nThis project uses `language \"moriarty/1\"` and `moriarty-core/1` local reference execution.\nThe TEST asset and accounts are examples. Simulation does not settle a ledger transaction or produce a proof.\n\nThe commands below use the Rust CLI that created this project. If you move that executable, update its absolute path below.\nRun these commands from this directory in a POSIX shell:\n\n```sh\n{executable} check source.mori --json\n{executable} simulate source.mori --action {action} --scenario scenario.json\n{executable} test .\n```\n\nThe manifest checks the complete expected post-state and ordered effects.\n\nInitialization requires a new destination and an existing parent directory.\nAn I/O failure can leave an incomplete project. Choose a new destination before retrying.\n"
    );
    Ok(vec![
        ("source.mori", source.as_bytes().to_vec()),
        ("scenario.json", format!("{scenario}\n").into_bytes()),
        ("mori-tests.json", format!("{manifest}\n").into_bytes()),
        ("README.md", readme.into_bytes()),
    ])
}

pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    // Keep generated commands on this Rust CLI even when another `mori` is on PATH.
    // Quote the complete absolute executable as one POSIX shell word.
    let executable = std::env::current_exe().map_err(super::core_io::operation)?;
    let executable = executable.to_str().ok_or_else(|| {
        super::core_io::operation("starter commands require a UTF-8 Rust executable path")
    })?;
    let executable = format!("'{}'", executable.replace('\'', "'\\''"));
    let files = project(args.template, &executable)?;
    // create_dir atomically refuses existing files, directories and symlinks.
    // Do not create ancestors or remove this directory after a later failure.
    std::fs::create_dir(&args.dir).map_err(|source| CliError::Write {
        path: args.dir.clone(),
        source,
    })?;
    for (name, bytes) in files {
        let path = args.dir.join(name);
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|source| CliError::Write {
                path: path.clone(),
                source,
            })?;
        file.write_all(&bytes)
            .map_err(|source| CliError::Write { path, source })?;
    }
    shell.status("Created", args.dir.display());
    Ok(ExitCode::SUCCESS)
}

```

## COMPLETE exact DevEx2file patch (full init test also physical) SHA256 53e6f8ebaeff7828b2fe9843b16d54af5cc44a3a5a9995fec6c904564b2ae76f
```
--- a/crates/mori/src/commands/init.rs
+++ b/crates/mori/src/commands/init.rs
@@ -47,7 +47,7 @@
 
 /// Prepare all bytes before reserving the destination. Expectations are literal
 /// financial examples, never outputs obtained from running the interpreter.
-fn project(template: Template) -> super::Result<Vec<(&'static str, Vec<u8>)>> {
+fn project(template: Template, executable: &str) -> super::Result<Vec<(&'static str, Vec<u8>)>> {
     use super::core_io::operation;
     let (source, action, amount, alice, bob) = match template {
         Template::Transfer => (TRANSFER, "pay", "25", "75", "25"),
@@ -90,7 +90,7 @@
     let scenario = mori_core::json::export(&scenario).map_err(operation)?;
     let manifest = mori_core::json::export(&manifest).map_err(operation)?;
     let readme = format!(
-        "# Moriarty starter\n\nThis project uses `language \"moriarty/1\"` and local reference execution.\nThe TEST asset and accounts are examples. Simulation does not settle a ledger transaction or produce a proof.\n\nRun these commands from this directory with `mori` on your PATH:\n\n```sh\nmori check source.mori --json\nmori simulate source.mori --action {action} --scenario scenario.json\nmori test .\n```\n\nThe manifest checks the complete expected post-state and ordered effects.\n\nInitialization requires a new destination and an existing parent directory.\nAn I/O failure can leave an incomplete project. Choose a new destination before retrying.\n"
+        "# Moriarty starter\n\nThis project uses `language \"moriarty/1\"` and `moriarty-core/1` local reference execution.\nThe TEST asset and accounts are examples. Simulation does not settle a ledger transaction or produce a proof.\n\nThe commands below use the Rust CLI that created this project. If you move that executable, update its absolute path below.\nRun these commands from this directory in a POSIX shell:\n\n```sh\n{executable} check source.mori --json\n{executable} simulate source.mori --action {action} --scenario scenario.json\n{executable} test .\n```\n\nThe manifest checks the complete expected post-state and ordered effects.\n\nInitialization requires a new destination and an existing parent directory.\nAn I/O failure can leave an incomplete project. Choose a new destination before retrying.\n"
     );
     Ok(vec![
         ("source.mori", source.as_bytes().to_vec()),
@@ -101,7 +101,14 @@
 }
 
 pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
-    let files = project(args.template)?;
+    // Keep generated commands on this Rust CLI even when another `mori` is on PATH.
+    // Quote the complete absolute executable as one POSIX shell word.
+    let executable = std::env::current_exe().map_err(super::core_io::operation)?;
+    let executable = executable.to_str().ok_or_else(|| {
+        super::core_io::operation("starter commands require a UTF-8 Rust executable path")
+    })?;
+    let executable = format!("'{}'", executable.replace('\'', "'\\''"));
+    let files = project(args.template, &executable)?;
     // create_dir atomically refuses existing files, directories and symlinks.
     // Do not create ancestors or remove this directory after a later failure.
     std::fs::create_dir(&args.dir).map_err(|source| CliError::Write {
--- a/crates/mori/tests/init.rs
+++ b/crates/mori/tests/init.rs
@@ -321,3 +321,75 @@
         assert!(report(&o).get("execution").is_none());
     }
 }
+
+#[cfg(unix)]
+#[test]
+fn generated_commands_use_the_creating_rust_cli_without_path_lookup() {
+    use std::os::unix::fs::PermissionsExt;
+
+    // Resolve the host tool before clearing PATH in the external project children.
+    let shell = std::env::var_os("PATH")
+        .and_then(|path| {
+            std::env::split_paths(&path)
+                .map(|directory| directory.join("sh"))
+                .find(|candidate| {
+                    fs::metadata(candidate).is_ok_and(|metadata| {
+                        metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
+                    })
+                })
+        })
+        .expect("the test host PATH must supply a POSIX sh");
+    let shell = fs::canonicalize(shell).expect("resolve the host shell before changing directory");
+    let f = Files::new();
+    let tools = f.path("Rust CLI's tools 🧪");
+    fs::create_dir(&tools).unwrap();
+    let executable = tools.join("mori");
+    fs::copy(env!("CARGO_BIN_EXE_mori"), &executable).unwrap();
+    for template in ["transfer", "repay"] {
+        let dir = f.path(template);
+        let mut c = Command::new(&executable);
+        c.current_dir(&f.0)
+            .env("PATH", "")
+            .arg("init")
+            .arg(&dir)
+            .args(["--template", template]);
+        success(&invoke(c));
+        let readme = fs::read_to_string(dir.join("README.md")).unwrap();
+        let commands = readme
+            .split_once("```sh\n")
+            .unwrap()
+            .1
+            .split_once("```")
+            .unwrap()
+            .0;
+        let mut c = Command::new(&shell);
+        c.current_dir(&dir)
+            .env("PATH", "")
+            .args(["-c", &format!("set -e\n{commands}")]);
+        let o = invoke(c);
+        success(&o);
+        let reports: Vec<Value> = serde_json::Deserializer::from_slice(&o.stdout)
+            .into_iter()
+            .collect::<Result<_, _>>()
+            .unwrap();
+        assert_eq!(reports.len(), 3);
+        assert!(readme.contains("Rust CLI"));
+        assert!(readme.contains("moriarty-core/1"));
+        assert_eq!(reports[0]["status"], "accepted");
+        assert_eq!(reports[0]["language"], "moriarty/1");
+        assert_eq!(reports[1]["status"], "accepted");
+        assert_eq!(reports[1]["qualification"], "local-reference-execution");
+        assert_eq!(reports[1]["execution"]["semantics"], "moriarty-core/1");
+        let (post, effects) = if template == "transfer" {
+            expected("25", "75", "25", None)
+        } else {
+            expected("30", "70", "30", Some("70"))
+        };
+        assert_eq!(reports[1]["execution"]["post"], post);
+        assert_eq!(reports[1]["execution"]["effects"], effects);
+        assert_eq!(reports[2]["status"], "passed");
+        assert_eq!(reports[2]["cases"][0]["actual"]["Ok"]["post"], post);
+        assert_eq!(reports[2]["cases"][0]["actual"]["Ok"]["effects"], effects);
+        assert_eq!(project_bytes(&dir).len(), 4);
+    }
+}

```

## Preserved original source/result evidence and guard
Original187input packet cdd664ec47b614db6c5ad7c8786456a28d4607bdf451a1f7250ec9a8b2b5e13c is nested. Ordinary WHOLE0ad2c7503d93fee2e526d33432c59174eeb266763c0922e7562800ebf1b22bf6 and designated security WHOLE0f61a01ba43d4c4c7d2252c3f66d56fbdec52c417ace4cf54391cab2102adb87 each preserve2complete objects, required6fields identical, one adds only toolAction/toolSummary;1fresh context each, no canonical model identity. Both say approve-exact-source-result-and-guarded-merge under unchanged-e3 guard. No old guard is silently waived. Scope:3librarytests+7CLItests+2publicRuststdout->actualTSconsumer tests passed, onebinarybuild, rootcleanup/sourceguards. Binary558b43b66bb1adfae4878460b54b20cb82e88f5ca0717f7b7e7ffdca31344e2b /Node24.21.0. Original independent financial64a6/nominalbdd550 freezes and separate postcapture104matched/20unasserted124coverage comparison preserved; fullwire/DTO/agreement/work and unverified external bindings/premises remain open. Captures remain observations of8267, not a newly executed merged tree or newly produced binary. No retest requested solely for source paths outside selected command graph.
A finite revised guard may accept only the EXACT root-pinned intervening path/blob allowlist and unchanged incoming39pins/selected consumer graph/dependencies; if actual callgraph/compilation/initialization/manifest effect changes the claimed consumer predicate, identify it concretely and require its real validation instead. Root must explicitly adopt any revised guard after both reviewers agree. No blanket future change, test reuse for changed predicates, native/proof/VM/key/ledger/parent acceptance, new runtime or public developer permission rule is allowed.
