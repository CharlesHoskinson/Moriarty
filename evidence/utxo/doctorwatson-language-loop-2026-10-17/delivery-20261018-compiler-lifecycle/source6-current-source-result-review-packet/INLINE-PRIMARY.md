# Fresh actual-formatted-source + four actual ordinary results review
Exact source8267ad950742c94f74fd871c266fa995c4e9e893/treea936ca2a3383045b38e90cc3ab867ae948fa64e1, base maine3eb6d12aba0c301d32b278bf9bdd6f98155308f. Physical source-after/39files matches every root CURRENT-SOURCE.json pin, Cargo.lock f47f1f3e04111a50134656dce0e9d278e19a6c162aaf06cdd5f79fceead68ff3. Previous sourcea1 bytes differ by pinned rustfmt files and retained previously merged financial guideline paragraph: actual-check-cycle/INTEGRATION-CHANGES.json and INTEGRATION-FORMATTING.diff show exact differences. This is a FRESH full current-source+actual-result review, no transfer of old source approval. Physical historical sourcea1/owner evidence retained for identity/scope only. Start-time CURRENT-SOURCE executionStatus UNRUN is superseded by final RESULT.json receipts below; no new product execution by packet author.

## COMPLETE actual formatted Rust crates/mori/src/cli.rs SHA256 3c16445a29ec536588efd1613d0782834e36dbdcfc11d874b8fa0f4a1d4033d2
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

## COMPLETE actual formatted Rust crates/mori/src/commands/expand/source6.rs SHA256 e38a9659b993ea15da96931ba1fed0f34cab796b8e8efaeb3a6f8e27a5e4aca7
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

## COMPLETE actual formatted Rust crates/mori/src/commands/expand.rs SHA256 f7ff48dcaece95b05392e25ad3a9dff66dd736f839ceaa2928bf9d768aedc320
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

## COMPLETE actual formatted Rust crates/mori/tests/source6_expand.rs SHA256 93fdf22def77d85a923f3c91d6324509dec4d65405c1a7548cbe830a125d6a25
```
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

#[test]
fn checked_beta_reaches_real_expansion_library_and_raw_stdout() {
    let files = Files::new();
    let check = files.invoke(&["check", "invoice.mori", "--json"]);
    assert_eq!(check.status.code(), Some(0));
    assert!(check.stderr.is_empty());
    let report: Value = serde_json::from_slice(&check.stdout).unwrap();
    assert_eq!(report["status"], "accepted");
    assert_eq!(report["actions"][0]["name"], "pay");
    assert_eq!(report["actions"][0]["support"], "local_s0");

    let expansion = files.invoke(&source6_arguments());
    assert_eq!(expansion.status.code(), Some(0));
    assert_eq!(expansion.stdout.as_slice(), fixture!("invoice.source6"));
    assert_eq!(expansion.stderr.as_slice(), fixture!("source6.stderr.txt"));
}

#[test]
fn explicit_source6_quiet_output_is_raw_source6_text() {
    let files = Files::new();
    let mut arguments = vec!["--quiet"];
    arguments.extend(source6_arguments());
    let expansion = files.invoke(&arguments);
    assert_eq!(expansion.status.code(), Some(0));
    assert_eq!(expansion.stdout.as_slice(), fixture!("invoice.source6"));
    assert!(expansion.stderr.is_empty());
}

#[test]
fn core_default_retains_complete_program_bytes() {
    let files = Files::new();
    let output = files.invoke(&[
        "expand",
        "core.mori",
        "--action",
        "noop",
        "--scenario",
        "core.json",
    ]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout.as_slice(), fixture!("core.stdout.json"));
    assert!(output.stderr.is_empty());
}

#[test]
fn beta_default_retains_core_parser_refusal_boundary() {
    let files = Files::new();
    let output = files.invoke(&[
        "expand",
        "invoice.mori",
        "--action",
        "pay",
        "--scenario",
        "scenario.json",
    ]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("mori::core::syntax"));
}

#[test]
fn source_action_selection_precedes_scenario_file_read() {
    let files = Files::new();
    for (source, action, expected) in [
        (
            "invoice.mori",
            "absent_action",
            "mori::source6::unknown_action",
        ),
        (
            "repayment.mori",
            "repay_loan",
            "mori::source6::unsupported_operation",
        ),
        (
            "oracle.mori",
            "read_fixing",
            "mori::source6::unsupported_operation",
        ),
    ] {
        let output = files.invoke(&[
            "expand",
            source,
            "--action",
            action,
            "--scenario",
            "absent.json",
            "--emit-source6-transfer",
        ]);
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains(expected), "{stderr}");
        assert!(!stderr.contains("could not read"));
    }
}

#[test]
fn scenario_profile_and_subset_have_explicit_consumer_diagnostics() {
    let files = Files::new();
    let base: Value = serde_json::from_slice(fixture!("invoice.scenario.json")).unwrap();
    let foreign: Value = serde_json::from_slice(fixture!("core.scenario.json")).unwrap();
    let mut with_override = base.clone();
    with_override["candidate_effects"] = serde_json::json!([]);
    let mut with_obligation = base.clone();
    with_obligation["obligation"] = serde_json::json!({});
    let mut missing_cell = base;
    missing_cell["balances"].as_array_mut().unwrap().pop();
    for (scenario, expected) in [
        (foreign, "mori::command::source6-scenario-profile"),
        (with_override, "mori::command::source6-scenario-unsupported"),
        (
            with_obligation,
            "mori::command::source6-scenario-unsupported",
        ),
        (missing_cell, "mori::command::source6-scenario-cells"),
    ] {
        fs::write(
            files.0.join("scenario.json"),
            serde_json::to_vec(&scenario).unwrap(),
        )
        .unwrap();
        let output = files.invoke(&source6_arguments());
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains(expected));
    }
}

#[test]
fn diagnostic_presentation_escapes_filename_action_and_scenario_shape_errors() {
    let files = Files::new();
    let hostile_file = "invoice\u{1b}\u{202e}.mori";
    fs::write(files.0.join(hostile_file), fixture!("invoice.mori")).unwrap();

    let unknown = files.invoke(&[
        "expand",
        hostile_file,
        "--action",
        "pay\n\u{1b}\u{202e}",
        "--scenario",
        "absent.json",
        "--emit-source6-transfer",
    ]);
    assert_eq!(unknown.status.code(), Some(1));
    assert!(unknown.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(stderr.contains("mori::source6::unknown_action"), "{stderr}");
    assert!(stderr.contains(r"invoice\u{1b}\u{202e}.mori"), "{stderr}");
    assert!(
        stderr.contains(r"No checked action named `pay\n\u{1b}\u{202e}` was found."),
        "{stderr}"
    );
    assert!(!stderr.contains(['\u{1b}', '\u{202e}']), "{stderr}");

    let mut bad_serde: Value = serde_json::from_slice(fixture!("invoice.scenario.json")).unwrap();
    bad_serde["replay"] = serde_json::json!("bad\u{1b}\u{202e}");
    fs::write(
        files.0.join("scenario.json"),
        serde_json::to_vec(&bad_serde).unwrap(),
    )
    .unwrap();
    let shape = files.invoke(&source6_arguments());
    assert_eq!(shape.status.code(), Some(1));
    assert!(shape.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&shape.stderr);
    assert!(
        stderr.contains("mori::command::source6-scenario-shape"),
        "{stderr}"
    );
    assert!(stderr.contains(r"bad\u{1b}\u{202e}"), "{stderr}");
    assert!(!stderr.contains(['\u{1b}', '\u{202e}']), "{stderr}");

    let mut bad_domain: Value = serde_json::from_slice(fixture!("invoice.scenario.json")).unwrap();
    bad_domain["domain"] = serde_json::json!("Other");
    fs::write(
        files.0.join("scenario.json"),
        serde_json::to_vec(&bad_domain).unwrap(),
    )
    .unwrap();
    let mismatch = files.invoke(&[
        "expand",
        hostile_file,
        "--action",
        "pay",
        "--scenario",
        "scenario.json",
        "--emit-source6-transfer",
    ]);
    assert_eq!(mismatch.status.code(), Some(1));
    assert!(mismatch.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&mismatch.stderr);
    assert!(stderr.contains("mori::source6::scenario_shape"), "{stderr}");
    assert!(stderr.contains(r"invoice\u{1b}\u{202e}.mori"), "{stderr}");
    assert!(!stderr.contains(['\u{1b}', '\u{202e}']), "{stderr}");
}

```

## COMPLETE actual formatted Rust crates/mori-source6/src/diagnostics.rs SHA256 d209620f2444ef9a06c3946c89b7ea3a5db85a38620c194995d0be316ecfcbec
```
use mori_diagnostics::MoriDiagnostic;
use mori_span::Span;

fn text(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    for character in value.chars() {
        if character.is_ascii() && !character.is_control() {
            result.push(character);
        } else {
            result.extend(character.escape_default());
        }
    }
    result
}

pub fn action(span: Span, name: &str) -> MoriDiagnostic {
    let name = text(name);
    MoriDiagnostic::error(format!("No checked action named `{name}` was found."))
        .with_code("mori::source6::unknown_action")
        .with_label(span.primary_label("select an action declared in this agreement"))
        .with_help("Pass the name after `action`, for example `pay`.")
}

pub fn unsupported(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This Source/6 expansion slice supports only beta `transfer` actions.")
        .with_code("mori::source6::unsupported_operation")
        .with_label(span.primary_label("this checked operation has no expansion in this slice"))
        .with_help("Use the existing TypeScript bridge for other local S0 operations; family operations remain specified only.")
}

pub fn shape(span: Span, expected: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("Source/6 expansion expected {expected}."))
        .with_code("mori::source6::checked_shape")
        .with_label(
            span.primary_label("the checked representation does not match the transfer mapping"),
        )
        .with_help(
            "The expansion must consume the current checker output without changing its meaning.",
        )
}

pub fn scenario(span: Span, field: &str, expected: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "Local scenario field `{field}` must contain {expected}."
    ))
    .with_code("mori::source6::scenario_shape")
    .with_label(span.primary_label("scenario formation for this action; cells remain local claims"))
    .with_help(
        "Supply the explicit local transfer cells in owner, recipient and fee-recipient order.",
    )
}

pub fn bound(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("Generated Source/6 exceeds its 65,536-byte presentation bound.")
        .with_code("mori::source6::source_bound")
        .with_label(
            span.primary_label("expansion cannot be represented within Source/6's source limit"),
        )
        .with_help("Use a proposal that fits the existing Source/6 profile.")
}

```

## COMPLETE actual formatted Rust crates/mori-source6/src/lib.rs SHA256 f89e8b91241742a34ec20545411bfa24f1c4833685ce0fd39290dec3a995af4a
```
//! Local Source/6 presentation for the closed beta transfer slice.
//!
//! This crate emits a proposal. Its scenario cells and declared hashes remain
//! claims; it performs no Core/5 preparation, signing, proof or ledger action.
//! The generated text must be consumed by the existing Source/6 lowerer before
//! any claim of Source/6 formation or Core/5 preparation.

#[cfg(feature = "source6-transfer-slice")]
mod diagnostics;
#[cfg(feature = "source6-transfer-slice")]
mod transfer;

#[cfg(feature = "source6-transfer-slice")]
pub use transfer::{
    Allowance, Balance, LocalTransferScenario, Replay, SelectedTransfer, TransferExpansion,
    expand_transfer, select_transfer,
};

```

## COMPLETE actual formatted Rust crates/mori-source6/src/transfer.rs SHA256 60b76bb0c4ff4b588db54793b32cbf5b108af2e98e340b8b60de46a6c1144c08
```
use mori_ast::NodeIdx;
use mori_checker::{Checked, Support, ValueIdx, ValueTag};
use mori_diagnostics::MoriDiagnostic;
use mori_lexer::{TokenIdx, TokenKind, decode_string};
use mori_span::Span;

use crate::diagnostics;

type Result<T> = std::result::Result<T, MoriDiagnostic>;

/// A balance cell supplied as an unverified local claim.
#[derive(Debug, Clone, Copy)]
pub struct Balance<'a> {
    pub account: &'a str,
    pub amount: u128,
}

/// Explicit opening counters; expansion never resets or consumes them.
#[derive(Debug, Clone, Copy)]
pub struct Allowance<'a> {
    pub owner: &'a str,
    pub remaining: u128,
    pub spent: u128,
}

#[derive(Debug, Clone, Copy)]
pub enum Replay {
    Unused,
    Consumed,
}

/// Typed local scenario for the default transfer proposal only.
///
/// Balance order is owner, recipient, fee recipient. All three cells are
/// supplied even for a zero fee. This is not a JSON wire format, authenticated
/// snapshot or candidate-effects override API.
#[derive(Debug, Clone, Copy)]
pub struct LocalTransferScenario<'a> {
    pub domain: &'a str,
    pub asset: &'a str,
    pub head: &'a str,
    pub predecessor: &'a str,
    pub round: u128,
    pub balances: [Balance<'a>; 3],
    pub allowance: Allowance<'a>,
    pub replay: Replay,
    pub work_remaining: u128,
    pub work_spent: u128,
    pub post_head: &'a str,
}

/// Generated source text, with no Core result or published post-state.
#[derive(Debug, PartialEq, Eq)]
pub struct TransferExpansion {
    pub source6: String,
    pub qualification: &'static str,
}

/// A source-selected transfer, produced only after the existing frontend checks.
/// The owned presentation terms permit scenario I/O to happen after selection.
pub struct SelectedTransfer {
    terms: Terms,
    span: Span,
}

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

fn field(checked: &Checked<'_>, value: ValueIdx, name: &str) -> Result<ValueIdx> {
    checked
        .values
        .pairs(value)
        .find(|&(key, _)| checked.ast.token_text(TokenIdx::new(usize::from(key))) == name)
        .map(|(_, field)| field)
        .ok_or_else(|| {
            diagnostics::shape(
                checked.ast.span(checked.values.node(value)),
                &format!("the checked `{name}` field"),
            )
        })
}

fn string(checked: &Checked<'_>, value: ValueIdx) -> Result<String> {
    if checked.values.tag(value) != ValueTag::String {
        return Err(diagnostics::shape(
            checked.ast.span(checked.values.node(value)),
            "a checked string",
        ));
    }
    let token = TokenIdx::new(usize::from(checked.values.rows.a()[value.index()]));
    Ok(decode_string(checked.ast.token_text(token)))
}

fn number(checked: &Checked<'_>, value: ValueIdx, tag: ValueTag) -> Result<u128> {
    if checked.values.tag(value) != tag {
        return Err(diagnostics::shape(
            checked.ast.span(checked.values.node(value)),
            "a checked scalar or nominal quantity in its declared role",
        ));
    }
    Ok(checked.values.amount(value))
}

fn id(checked: &Checked<'_>, entity: ValueIdx) -> Result<String> {
    string(checked, field(checked, entity, "id")?)
}

fn call_name(checked: &Checked<'_>, value: ValueIdx) -> String {
    let mut token = checked.ast.main_token(checked.values.node(value));
    let mut name = checked.ast.token_text(token).to_owned();
    while checked.ast.tokens.kind(checked.ast.token_after(token, 1)) == TokenKind::Dot {
        token = checked.ast.token_after(token, 2);
        name.push('.');
        name.push_str(checked.ast.token_text(token));
    }
    name
}

fn extract(checked: &Checked<'_>, action: &str) -> Result<(Terms, Span)> {
    let ast = checked.ast;
    let index = checked
        .actions
        .node()
        .iter()
        .position(|&node| ast.token_text(ast.token_after(ast.main_token(node), 1)) == action);
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
    let asset = field(checked, intent, "asset")?;
    let validity = field(checked, intent, "valid")?;
    let quantity =
        |record, name| number(checked, field(checked, record, name)?, ValueTag::Quantity);
    let scalar = |record, name| number(checked, field(checked, record, name)?, ValueTag::Scalar);
    let text = |name| string(checked, field(checked, intent, name)?);
    let value = quantity(operation, "value")?;
    let fee = quantity(operation, "fee")?;
    let gross = value
        .checked_add(fee)
        .ok_or_else(|| diagnostics::shape(span, "a UInt128 transfer total"))?;
    Ok((
        Terms {
            agreement: ast.token_text(ast.main_token(NodeIdx::ROOT)).to_owned(),
            domain: id(checked, field(checked, intent, "domain")?)?,
            asset: id(checked, asset)?,
            scale: scalar(asset, "scale")?,
            signer: id(checked, field(checked, intent, "signer")?)?,
            owner: id(checked, field(checked, operation, "from")?)?,
            recipient: id(checked, field(checked, operation, "to")?)?,
            fee_recipient: id(checked, field(checked, operation, "fee_to")?)?,
            key: text("key")?,
            nonce: text("nonce")?,
            pre_head: text("pre_head")?,
            valid_from: scalar(validity, "from")?,
            valid_to: scalar(validity, "to")?,
            gross_cap: quantity(intent, "gross_cap")?,
            fee_cap: quantity(intent, "fee_cap")?,
            net_floor: quantity(intent, "net_floor")?,
            value,
            fee,
            gross,
            source_hash: text("source_hash")?,
            policy_digest: text("policy_digest")?,
        },
        span,
    ))
}

fn validate_scenario(terms: &Terms, s: &LocalTransferScenario<'_>, span: Span) -> Result<()> {
    for (field, actual, expected) in [
        ("domain", s.domain, terms.domain.as_str()),
        ("asset", s.asset, terms.asset.as_str()),
        ("allowance.owner", s.allowance.owner, terms.signer.as_str()),
    ] {
        if actual != expected {
            return Err(diagnostics::scenario(span, field, expected));
        }
    }
    for (index, (balance, expected)) in s
        .balances
        .iter()
        .zip([
            terms.owner.as_str(),
            terms.recipient.as_str(),
            terms.fee_recipient.as_str(),
        ])
        .enumerate()
    {
        if balance.account != expected {
            return Err(diagnostics::scenario(
                span,
                &format!("balances[{index}].account"),
                expected,
            ));
        }
    }
    for (field, text) in [
        ("head", s.head),
        ("predecessor", s.predecessor),
        ("post_head", s.post_head),
    ] {
        if text.is_empty() || text.len() > mori_lexer::MAX_STRING_BYTES {
            return Err(diagnostics::scenario(
                span,
                field,
                "nonempty text of at most 1024 UTF-8 bytes",
            ));
        }
    }
    if s.allowance
        .remaining
        .checked_add(s.allowance.spent)
        .is_none()
        || s.work_remaining.checked_add(s.work_spent).is_none()
    {
        return Err(diagnostics::scenario(
            span,
            "counters",
            "opening totals that fit UInt128",
        ));
    }
    Ok(())
}

fn quote(text: &str) -> String {
    serde_json::to_string(text).expect("serializing a UTF-8 string cannot fail")
}

fn append_line(out: &mut String, line: String, span: Span) -> Result<()> {
    if out.len() + line.len() + 1 > mori_lexer::MAX_SOURCE_BYTES {
        return Err(diagnostics::bound(span));
    }
    out.push_str(&line);
    out.push('\n');
    Ok(())
}

fn emit(t: &Terms, s: &LocalTransferScenario<'_>, span: Span) -> Result<String> {
    let mut out = String::new();
    macro_rules! line {
        ($($arg:tt)*) => {
            append_line(&mut out, format!($($arg)*), span)?
        };
    }
    let statement = format!(
        "transfer from {} to {} fee_to {} value {} fee {}",
        t.owner, t.recipient, t.fee_recipient, t.value, t.fee,
    );
    line!("profile \"moriarty-financial-agreement-source/6\";");
    line!("agreement {} {{", t.agreement);
    line!("domain {};", t.domain);
    line!("settlement {} scale {};", t.asset, t.scale);
    line!(
        "selected TransferLiteralFee source_hash {} digest {};",
        quote(&t.source_hash),
        quote(&t.policy_digest)
    );
    line!("intent {{");
    line!("signer {} key {};", t.signer, quote(&t.key));
    line!("nonce {};", quote(&t.nonce));
    line!("pre_head {};", quote(&t.pre_head));
    line!("valid {}..{};", t.valid_from, t.valid_to);
    line!("gross_cap {};", t.gross_cap);
    line!("fee_cap {};", t.fee_cap);
    line!("net_floor {};", t.net_floor);
    line!("failure success_only;");
    line!("signed_action {statement};");
    for field in [
        "observations",
        "disclosures",
        "retained_effects",
        "retained_duties",
    ] {
        line!("{field} empty;");
    }
    for field in ["delegation", "recovery"] {
        line!("{field} none;");
    }
    line!("}}");
    line!("authenticated {{");
    line!("head {};", quote(s.head));
    line!("predecessor {};", quote(s.predecessor));
    line!("round {};", s.round);
    for balance in &s.balances {
        line!("balance {} {};", balance.account, balance.amount);
    }
    line!(
        "allowance {} remaining {} spent {};",
        s.allowance.owner,
        s.allowance.remaining,
        s.allowance.spent
    );
    let replay = match s.replay {
        Replay::Unused => "unused",
        Replay::Consumed => "consumed",
    };
    line!("replay {replay};");
    line!("work_remaining {};", s.work_remaining);
    line!("work_spent {};", s.work_spent);
    line!("}}");
    line!("submit {statement};");
    line!("effects {{");
    line!("debit {} {};", t.owner, t.gross);
    line!("credit {} {};", t.recipient, t.value);
    if t.fee > 0 {
        line!("credit {} {};", t.fee_recipient, t.fee);
    }
    line!("use_allowance {} {};", t.signer, t.gross);
    line!("use_replay {};", quote(&t.nonce));
    line!(
        "advance_head {} {};",
        quote(&t.pre_head),
        quote(s.post_head)
    );
    line!("}}");
    line!("post_head {};", quote(s.post_head));
    line!("}}");
    Ok(out)
}

```

## COMPLETE actual formatted Rust crates/mori-source6/tests/transfer.rs SHA256 d961edc47ef99b86f0ff23db40bd66b1971a2b98a0d8698b67c6dc65543f9cee
```
#![cfg(feature = "source6-transfer-slice")]

use mori_source6::{Allowance, Balance, LocalTransferScenario, Replay, expand_transfer};

fn invoice_scenario() -> LocalTransferScenario<'static> {
    LocalTransferScenario {
        domain: "Midnight",
        asset: "A",
        head: "h0",
        predecessor: "genesis",
        round: 1,
        balances: [
            Balance {
                account: "Owner",
                amount: 10_000,
            },
            Balance {
                account: "Recipient",
                amount: 0,
            },
            Balance {
                account: "Fee",
                amount: 0,
            },
        ],
        allowance: Allowance {
            owner: "Owner",
            remaining: 10_000,
            spent: 0,
        },
        replay: Replay::Unused,
        work_remaining: 10,
        work_spent: 0,
        post_head: "h1",
    }
}

#[test]
fn literal_decimal_transfer_has_complete_source6_presentation() {
    let source = include_str!("fixtures/invoice.mori");
    let expansion = expand_transfer(source, "pay", &invoice_scenario()).unwrap();
    assert_eq!(expansion.qualification, "local-stipulation-only");
    assert_eq!(expansion.source6, include_str!("fixtures/invoice.source6"));
    insta::with_settings!({
        description => format!("Code:\n\n{source}"),
        omit_expression => true,
    }, {
        insta::assert_snapshot!("invoice_source6", expansion.source6);
    });
}

#[test]
fn referenced_quantity_sum_and_fee_use_exact_atomic_values() {
    let source = include_str!("fixtures/workshop.mori");
    let scenario = LocalTransferScenario {
        head: "s0",
        predecessor: "earlier",
        round: 3,
        balances: [
            Balance {
                account: "Owner",
                amount: 1000,
            },
            Balance {
                account: "Recipient",
                amount: 5,
            },
            Balance {
                account: "Fee",
                amount: 2,
            },
        ],
        allowance: Allowance {
            owner: "Owner",
            remaining: 900,
            spent: 100,
        },
        work_remaining: 9,
        work_spent: 4,
        post_head: "s1",
        ..invoice_scenario()
    };
    let expansion = expand_transfer(source, "pay", &scenario).unwrap();
    assert_eq!(expansion.qualification, "local-stipulation-only");
    assert_eq!(expansion.source6, include_str!("fixtures/workshop.source6"));
    insta::with_settings!({
        description => format!("Code:\n\n{source}"),
        omit_expression => true,
    }, {
        insta::assert_snapshot!("workshop_source6", expansion.source6);
    });
}

#[test]
fn unknown_action_diagnostic_escapes_control_and_non_ascii_characters() {
    let source = include_str!("fixtures/invoice.mori");
    let errors = expand_transfer(source, "pay's\n\u{1b}\u{202e}", &invoice_scenario())
        .expect_err("unknown action must fail");
    assert_eq!(errors.len(), 1);
    let message = errors[0].to_string();
    assert_eq!(
        message,
        r"No checked action named `pay's\n\u{1b}\u{202e}` was found.",
    );
}

```

## COMPLETE actual TS consumer experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts SHA256 f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7
```
/** Provisional, closed Source/6 S0 presentation parser. No authentication occurs here. */
import {
  MIL4_S0_CORE, MIL4_S0_INTENT, MIL4_S0_SOURCE,
  type S0Effect, type S0Intent, type S0State,
} from './mil4-s0-core-v5.ts';

const U128 = (1n << 128n) - 1n;
const S128 = (1n << 127n) - 1n;
const encoder = new TextEncoder();
const RESERVED = new Set((
  'profile agreement unit party asset const state action requires let next emit ensures true false not and or domain settlement scale selected source_hash digest intent signer key nonce pre_head post_head valid gross_cap fee_cap net_floor failure success_only signed_action observations empty disclosures retained_effects retained_duties delegation none recovery authenticated head predecessor round balance allowance remaining spent obligation debtor creditor principal accrued outstanding settled status replay unused consumed work_remaining work_spent submit transfer from to fee_to value fee repay payer amount conversion identity effects debit credit set_obligation use_allowance use_replay advance_head'
).split(' '));

type TokenKind = 'word' | 'integer' | 'string' | 'punctuation' | 'eof';
interface Token { kind: TokenKind; text: string; value: string; start: number; end: number }
export class Source6Error extends Error {
  readonly code: string;
  readonly offset: number;
  constructor(code: string, offset: number, message: string) {
    super(message);
    this.name = 'Source6Error';
    this.code = code;
    this.offset = offset;
  }
}
function fail(code: string, offset: number, message: string): never {
  throw new Source6Error(code, offset, message);
}
function isSurrogate(value: number): boolean { return value >= 0xd800 && value <= 0xdfff; }
function scalarString(value: string, offset: number): void {
  for (let i = 0; i < value.length; i++) {
    const c = value.charCodeAt(i);
    if (c >= 0xd800 && c <= 0xdbff && i + 1 < value.length) {
      const low = value.charCodeAt(i + 1);
      if (low >= 0xdc00 && low <= 0xdfff) { i++; continue; }
    }
    if (isSurrogate(c)) fail('INVALID_SURROGATE', offset + encoder.encode(value.slice(0, i)).length, 'Lone UTF-16 surrogate');
  }
}
function lexical(source: string): Token[] {
  scalarString(source, 0);
  if (encoder.encode(source).length > 65536) fail('SOURCE_BOUND', 0, 'Source exceeds 65536 UTF-8 bytes');
  const tokens: Token[] = [];
  let i = 0;
  let byte = 0;
  const advance = (end: number): void => { byte += encoder.encode(source.slice(i, end)).length; i = end; };
  const emit = (kind: TokenKind, end: number, value = source.slice(i, end)): void => {
    if (tokens.length >= 8191) fail('TOKEN_BOUND', byte, 'Too many tokens');
    const start = byte;
    const raw = source.slice(i, end);
    advance(end);
    tokens.push({ kind, text: raw, value, start, end: byte });
  };
  while (i < source.length) {
    const c = source[i];
    if (/[ \t\r\n]/.test(c)) { advance(i + 1); continue; }
    if (source.startsWith('//', i)) {
      const next = source.indexOf('\n', i + 2);
      advance(next < 0 ? source.length : next); continue;
    }
    if (source.startsWith('/*', i)) {
      const next = source.indexOf('*/', i + 2);
      if (next < 0) fail('UNTERMINATED_COMMENT', byte, 'Unclosed block comment');
      advance(next + 2); continue;
    }
    if (c === '"') {
      let end = i + 1;
      let escaped = false;
      for (; end < source.length; end++) {
        const x = source[end];
        if (x === '"' && !escaped) { end++; break; }
        if (x === '\\' && !escaped) escaped = true;
        else escaped = false;
      }
      const raw = source.slice(i, end);
      if (!raw.endsWith('"') || raw.length < 2) fail('INVALID_STRING', byte, 'Unclosed string');
      let value: string;
      try { value = JSON.parse(raw) as string; }
      catch { fail('INVALID_STRING', byte, 'Invalid JSON string'); }
      scalarString(value, byte);
      if (encoder.encode(value).length > 1024) fail('STRING_BOUND', byte, 'Decoded string exceeds 1024 UTF-8 bytes');
      emit('string', end, value); continue;
    }
    if (/[0-9]/.test(c)) {
      let end = i + 1;
      while (end < source.length && /[0-9]/.test(source[end])) end++;
      const raw = source.slice(i, end);
      if (raw.length > 78) fail('INTEGER_BOUND', byte, 'Integer exceeds 78 digits');
      if (!/^(0|[1-9][0-9]*)$/.test(raw)) fail('INVALID_INTEGER', byte, 'Noncanonical integer');
      emit('integer', end); continue;
    }
    if (/[A-Za-z]/.test(c)) {
      let end = i + 1;
      while (end < source.length && /[A-Za-z0-9_]/.test(source[end])) end++;
      const next = source.codePointAt(end);
      if (next !== undefined && /[\p{L}\p{N}\p{Pc}\p{Mn}\p{Mc}]/u.test(String.fromCodePoint(next)))
        fail('NON_ASCII_IDENTIFIER', byte, 'Non-ASCII identifier');
      if (end - i > 64) fail('IDENTIFIER_BOUND', byte, 'Identifier exceeds 64 characters');
      emit('word', end); continue;
    }
    const point = source.codePointAt(i)!;
    if (/[\p{L}\p{N}\p{Pc}\p{Mn}\p{Mc}]/u.test(String.fromCodePoint(point)))
      fail('NON_ASCII_IDENTIFIER', byte, 'Non-ASCII identifier');
    if (source.startsWith('..', i)) { emit('punctuation', i + 2); continue; }
    if ('{};'.includes(c)) { emit('punctuation', i + 1); continue; }
    fail('UNEXPECTED_CHAR', byte, 'Unexpected source character');
  }
  tokens.push({ kind: 'eof', text: '', value: '', start: byte, end: byte });
  return tokens;
}

export type Source6Action =
  | { kind: 'Transfer'; from: string; to: string; feeTo: string; value: string; fee: string }
  | { kind: 'Repay'; obligation: string; payer: string; amount: string; conversion: 'identity' };
export interface Source6Obligation {
  id: string; debtor: string; creditor: string; asset: string;
  principal: string; accrued: string; outstanding: string; status: 'Outstanding';
}
export interface Source6Ast {
  profile: typeof MIL4_S0_SOURCE; programId: string; domain: string;
  settlement: { asset: string; scale: string };
  selected: { actionId: string; sourceHash: string; policyDigest: string };
  intent: {
    signer: string; keyRef: string; nonce: string; preHead: string;
    notBefore: string; notAfter: string; grossCap: string; feeCap: string; netFloor: string;
    signedAction: Source6Action; failure: 'success_only';
    observations: 'empty'; disclosures: 'empty'; retainedEffects: 'empty'; retainedDuties: 'empty';
    delegation: 'none'; recovery: 'none';
  };
  authenticated: {
    head: string; predecessor: string; round: string;
    balances: { account: string; amount: string }[];
    allowance: { owner: string; remaining: string; spent: string };
    obligation?: Source6Obligation; replay: 'unused' | 'consumed';
    workRemaining: string; workSpent: string;
  };
  submitted: { action: Source6Action; effects: S0Effect[]; postHead: string };
}

class Parser {
  private index = 0;
  private nodes = 0;
  private depth = 0;
  private readonly tokens: Token[];
  constructor(tokens: Token[]) { this.tokens = tokens; }
  private get here(): Token { return this.tokens[this.index]; }
  private node(): void {
    if (++this.nodes > 8192) fail('AST_BOUND', this.here.start, 'Too many AST nodes');
  }
  private enter(): void { if (++this.depth > 64) fail('DEPTH_BOUND', this.here.start, 'Nesting exceeds 64'); }
  private leave(): void { this.depth--; }
  private take(word: string, code = 'SOURCE6_SHAPE'): void {
    if (this.here.text !== word) fail(code, this.here.start, `Expected ${word}`);
    this.index++;
  }
  private effectTag(word: string): void {
    if (this.here.text !== word && this.here.kind === 'word'
        && !new Set(['debit', 'credit', 'set_obligation', 'use_allowance', 'use_replay', 'advance_head']).has(this.here.text))
      fail('SOURCE6_UNKNOWN_TAG', this.here.start, 'Unknown effect tag');
    this.take(word);
  }
  private id(): string {
    const token = this.here;
    if (token.kind !== 'word' || RESERVED.has(token.text)) fail('SOURCE6_SHAPE', token.start, 'Expected identifier');
    this.index++; return token.value;
  }
  private string(): string {
    const token = this.here;
    if (token.kind !== 'string') fail('SOURCE6_SHAPE', token.start, 'Expected string');
    if (token.value.length === 0) fail('SOURCE6_SHAPE', token.start, 'Empty opaque string');
    this.index++; return token.value;
  }
  private uint(max: bigint = U128): string {
    const token = this.here;
    if (token.kind !== 'integer') fail('SOURCE6_SHAPE', token.start, 'Expected integer');
    this.index++;
    if (BigInt(token.value) > max) fail('SOURCE6_RANGE', token.start, 'Integer exceeds nominal bound');
    return token.value;
  }
  private action(): Source6Action {
    this.node();
    if (this.here.text === 'transfer') {
      this.take('transfer'); this.take('from'); const from = this.id();
      this.take('to'); const to = this.id(); this.take('fee_to'); const feeTo = this.id();
      this.take('value'); const value = this.uint(S128); this.take('fee'); const fee = this.uint(S128);
      this.take(';'); return { kind: 'Transfer', from, to, feeTo, value, fee };
    }
    if (this.here.text === 'repay') {
      this.take('repay'); this.take('obligation'); const obligation = this.id();
      this.take('payer'); const payer = this.id(); this.take('amount'); const amount = this.uint(S128);
      this.take('conversion'); this.take('identity'); this.take(';');
      return { kind: 'Repay', obligation, payer, amount, conversion: 'identity' };
    }
    fail('SOURCE6_UNKNOWN_TAG', this.here.start, 'Unknown action');
  }
  private effects(action: Source6Action, asset: string): S0Effect[] {
    this.node(); this.enter(); this.take('effects'); this.take('{');
    const debit = (): S0Effect => { this.effectTag('debit'); const account = this.id(); const amount = this.uint(); this.take(';'); return { kind: 'Debit', account, asset, amount }; };
    const credit = (): S0Effect => { this.effectTag('credit'); const account = this.id(); const amount = this.uint(); this.take(';'); return { kind: 'Credit', account, asset, amount }; };
    const result: S0Effect[] = [debit(), credit()];
    if (action.kind === 'Transfer' && this.here.text === 'credit') result.push(credit());
    if (action.kind === 'Repay') {
      this.effectTag('set_obligation'); const id = this.id(); this.take('principal'); const principal = this.uint(S128);
      this.take('accrued'); const accrued = this.uint(S128); this.take('outstanding'); const outstanding = this.uint(S128);
      this.take('status');
      if (this.here.text !== 'outstanding' && this.here.text !== 'settled')
        fail('SOURCE6_SHAPE', this.here.start, 'Expected obligation status');
      const status = this.here.text === 'settled' ? 'Settled' : 'Outstanding'; this.index++; this.take(';');
      result.push({ kind: 'SetObligation', id, principal, accrued, outstanding, status });
    }
    this.effectTag('use_allowance'); const owner = this.id(); const amount = this.uint(); this.take(';');
    result.push({ kind: 'UseAllowance', owner, amount });
    this.effectTag('use_replay'); const key = this.string(); this.take(';'); result.push({ kind: 'UseReplay', key });
    this.effectTag('advance_head'); const predecessor = this.string(); const successor = this.string(); this.take(';');
    result.push({ kind: 'AdvanceHead', predecessor, successor }); this.take('}'); this.leave();
    return result;
  }
  parse(): Source6Ast {
    this.node(); this.take('profile', 'SOURCE6_VERSION');
    const profileToken = this.here;
    if (profileToken.kind !== 'string' || profileToken.text !== '"moriarty-financial-agreement-source/6"')
      fail('SOURCE6_VERSION', profileToken.start, 'Unsupported source profile');
    this.index++;
    this.take(';'); this.take('agreement'); const programId = this.id(); this.take('{'); this.enter();
    this.take('domain'); const domain = this.id(); this.take(';');
    this.take('settlement'); const asset = this.id(); this.take('scale'); const scale = this.uint(18n); this.take(';');
    this.take('selected'); const actionId = this.id(); this.take('source_hash'); const sourceHash = this.string();
    this.take('digest'); const policyDigest = this.string(); this.take(';');
    this.take('intent'); this.take('{'); this.enter(); this.node();
    this.take('signer'); const signer = this.id(); this.take('key'); const keyRef = this.string(); this.take(';');
    this.take('nonce'); const nonce = this.string(); this.take(';');
    this.take('pre_head'); const preHead = this.string(); this.take(';');
    this.take('valid'); const notBefore = this.uint(); this.take('..'); const notAfter = this.uint(); this.take(';');
    if (BigInt(notBefore) > BigInt(notAfter))
      fail('SOURCE6_RANGE', this.here.start, 'Validity lower bound exceeds upper bound');
    this.take('gross_cap'); const grossCap = this.uint(S128); this.take(';');
    this.take('fee_cap'); const feeCap = this.uint(S128); this.take(';');
    this.take('net_floor'); const netFloor = this.uint(S128); this.take(';');
    this.take('failure'); this.take('success_only'); this.take(';'); this.take('signed_action');
    const signedAction = this.action();
    for (const field of ['observations', 'disclosures', 'retained_effects', 'retained_duties']) {
      this.take(field); this.take('empty'); this.take(';');
    }
    this.take('delegation'); this.take('none'); this.take(';');
    this.take('recovery'); this.take('none'); this.take(';'); this.take('}'); this.leave();
    this.take('authenticated'); this.take('{'); this.enter(); this.node();
    this.take('head'); const head = this.string(); this.take(';');
    this.take('predecessor'); const predecessor = this.string(); this.take(';');
    this.take('round'); const round = this.uint(); this.take(';');
    const balances: { account: string; amount: string }[] = [];
    const balanceCount = signedAction.kind === 'Transfer' ? 3 : 2;
    for (let i = 0; i < balanceCount; i++) {
      this.take('balance', 'SOURCE6_CELL_SHAPE');
      balances.push({ account: this.id(), amount: this.uint() }); this.take(';');
    }
    if (this.here.text === 'balance')
      fail('SOURCE6_CELL_SHAPE', this.here.start, 'Too many balance rows');
    this.take('allowance'); const owner = this.id(); this.take('remaining'); const remaining = this.uint();
    this.take('spent'); const spent = this.uint(); this.take(';');
    let obligation: Source6Obligation | undefined;
    if (signedAction.kind === 'Repay') {
      this.take('obligation', 'SOURCE6_CELL_SHAPE'); const id = this.id(); this.take('{'); this.enter(); this.node();
      this.take('debtor'); const debtor = this.id(); this.take(';');
      this.take('creditor'); const creditor = this.id(); this.take(';');
      this.take('asset'); const obligationAsset = this.id(); this.take(';');
      this.take('principal'); const principal = this.uint(S128); this.take(';');
      this.take('accrued'); const accrued = this.uint(S128); this.take(';');
      this.take('outstanding'); const outstanding = this.uint(S128); this.take(';');
      this.take('status'); this.take('outstanding'); this.take(';'); this.take('}'); this.leave();
      obligation = { id, debtor, creditor, asset: obligationAsset, principal, accrued, outstanding, status: 'Outstanding' };
    }
    if (this.here.text === 'obligation')
      fail('SOURCE6_CELL_SHAPE', this.here.start, 'Unexpected obligation row');
    this.take('replay');
    if (this.here.text !== 'unused' && this.here.text !== 'consumed')
      fail('SOURCE6_SHAPE', this.here.start, 'Expected replay status');
    const replay = this.here.text as 'unused' | 'consumed'; this.index++; this.take(';');
    this.take('work_remaining'); const workRemaining = this.uint(); this.take(';');
    this.take('work_spent'); const workSpent = this.uint(); this.take(';'); this.take('}'); this.leave();
    this.take('submit'); const action = this.action(); const effects = this.effects(action, asset);
    this.take('post_head'); const postHead = this.string(); this.take(';'); this.take('}'); this.leave();
    if (this.tokens[this.index].kind !== 'eof') fail('SOURCE6_SHAPE', this.here.start, 'Trailing source');
    const expectedActionId = signedAction.kind === 'Transfer' ? 'TransferLiteralFee' : 'RepayAccrualFirst';
    if (actionId !== expectedActionId)
      fail('SOURCE6_PROFILE_UNSUPPORTED', this.here.start, 'Selected action is outside the S0 profile');
    const accounts = balances.map((row) => row.account);
    const expectedAccounts = signedAction.kind === 'Transfer'
      ? [signedAction.from, signedAction.to, signedAction.feeTo]
      : [signedAction.payer, obligation?.creditor];
    if (accounts.some((id, i) => id !== expectedAccounts[i]) || new Set(accounts).size !== accounts.length
        || owner !== signer || (signedAction.kind === 'Transfer' &&
          (signedAction.from !== signer || new Set([signedAction.from, signedAction.to, signedAction.feeTo]).size !== 3))
        || (signedAction.kind === 'Repay' && (!obligation || obligation.id !== signedAction.obligation
          || signedAction.payer !== signer || obligation.debtor !== signer || obligation.asset !== asset
          || obligation.creditor === signer)))
      fail('SOURCE6_CELL_SHAPE', this.here.start, 'Authenticated cells do not match action');
    if (BigInt(workRemaining) + BigInt(workSpent) > U128
        || BigInt(remaining) + BigInt(spent) > U128)
      fail('SOURCE6_RANGE', this.here.start, 'Counter total exceeds UInt128');
    if (obligation && BigInt(obligation.principal) + BigInt(obligation.accrued) !== BigInt(obligation.outstanding))
      fail('SOURCE6_RANGE', this.here.start, 'Obligation outstanding must equal principal plus accrued');
    return {
      profile: MIL4_S0_SOURCE, programId, domain, settlement: { asset, scale },
      selected: { actionId, sourceHash, policyDigest },
      intent: { signer, keyRef, nonce, preHead, notBefore, notAfter, grossCap, feeCap, netFloor,
        signedAction, failure: 'success_only', observations: 'empty', disclosures: 'empty',
        retainedEffects: 'empty', retainedDuties: 'empty', delegation: 'none', recovery: 'none' },
      authenticated: { head, predecessor, round, balances, allowance: { owner, remaining, spent },
        obligation, replay, workRemaining, workSpent },
      submitted: { action, effects, postHead },
    };
  }
}

/** Parse one exact Source/6 S0 document. Throws Source6Error on formation failure. */
export function parseSource6(source: string): Source6Ast { return new Parser(lexical(source)).parse(); }

export interface Source6Lowered {
  ast: Source6Ast; state: S0State; intent: S0Intent;
  submittedEffects: S0Effect[]; proposedPostHead: string;
}

/** Lower claims for local Core/5 preparation; no signed digest or external premise is manufactured. */
export function lowerSource6(ast: Source6Ast): Source6Lowered {
  const { authenticated: auth, intent: signed, submitted, settlement, selected } = ast;
  const replayKey = JSON.stringify([ast.domain, signed.signer, signed.nonce]);
  const state: S0State = {
    core: MIL4_S0_CORE, domain: ast.domain, asset: settlement.asset, head: auth.head,
    round: auth.round, workRemaining: auth.workRemaining, workSpent: auth.workSpent,
    balances: auth.balances.map((row) => ({ ...row })), allowances: [{ ...auth.allowance }],
    obligations: auth.obligation ? [{ ...auth.obligation }] : [],
    consumedReplay: auth.replay === 'consumed' ? [replayKey] : [],
  };
  const base = {
    version: MIL4_S0_INTENT, core: MIL4_S0_CORE, sourceProfile: MIL4_S0_SOURCE,
    programId: selected.actionId, sourceHash: selected.sourceHash, policyDigest: selected.policyDigest,
    keyRef: signed.keyRef, domain: ast.domain, asset: settlement.asset,
    signer: signed.signer, nonce: signed.nonce, preHead: signed.preHead,
    notBefore: signed.notBefore, notAfter: signed.notAfter,
    grossCap: signed.grossCap, feeCap: signed.feeCap, netFloor: signed.netFloor,
  };
  const action = signed.signedAction;
  const intent: S0Intent = action.kind === 'Transfer'
    ? { ...base, kind: 'Transfer', recipient: action.to, feeRecipient: action.feeTo,
        amount: action.value, fee: action.fee }
    : { ...base, kind: 'Repay', obligationId: action.obligation, amount: action.amount };
  // Source `use_replay` names a nonce. Core/5 compares its domain/signer/nonce tuple.
  const submittedEffects = submitted.effects.map((effect): S0Effect =>
    effect.kind === 'UseReplay'
      ? { kind: 'UseReplay', key: JSON.stringify([ast.domain, signed.signer, effect.key]) }
      : { ...effect });
  return { ast, state, intent, submittedEffects, proposedPostHead: submitted.postHead };
}

export function parseAndLowerSource6(source: string): Source6Lowered {
  return lowerSource6(parseSource6(source));
}

```

## COMPLETE actual TS consumer experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts SHA256 1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8
```
/** Provisional Source/6 to Core/5 local preparation. This module cannot admit a ledger stage. */
import { prepareMil4S0, type S0PreparedUnqualified, type S0Rejected } from './mil4-s0-core-v5.ts';
import {
  parseAndLowerSource6, Source6Error,
  type Source6Ast, type Source6Lowered,
} from './financial-agreement-source-v6-frontend.ts';

export type Source6S0Outcome =
  | { status: 'SourceRejected'; code: string; offset: number; publishedPost: null; publishedEffects: null }
  | { status: 'CoreRejected'; ast: Source6Ast; rejection: S0Rejected }
  | {
      status: 'PreparedUnqualified'; ast: Source6Ast; candidate: S0PreparedUnqualified;
      unverifiedBindings: readonly ['agreement-id', 'selected-program', 'asset-scale', 'authenticated-predecessor'];
    };

function sameAction(a: Source6Ast['intent']['signedAction'], b: Source6Ast['submitted']['action']): boolean {
  if (a.kind !== b.kind) return false;
  if (a.kind === 'Transfer' && b.kind === 'Transfer') {
    return a.from === b.from && a.to === b.to && a.feeTo === b.feeTo
      && a.value === b.value && a.fee === b.fee;
  }
  return a.kind === 'Repay' && b.kind === 'Repay'
    && a.obligation === b.obligation && a.payer === b.payer && a.amount === b.amount
    && a.conversion === b.conversion;
}

/** Parse and prepare one S0 stage without asserting source authentication or ledger acceptance. */
export function prepareSource6S0Unqualified(source: string): Source6S0Outcome {
  let lowered: Source6Lowered;
  try {
    lowered = parseAndLowerSource6(source);
  } catch (error) {
    if (!(error instanceof Source6Error)) throw error;
    return {
      status: 'SourceRejected', code: error.code, offset: error.offset,
      publishedPost: null, publishedEffects: null,
    };
  }
  const { ast, state, intent, submittedEffects, proposedPostHead } = lowered;
  const result = prepareMil4S0(state, intent, submittedEffects, proposedPostHead);
  if (result.status === 'Rejected' && result.judgment === 'stage') {
    return { status: 'CoreRejected', ast, rejection: result };
  }
  if (!sameAction(ast.intent.signedAction, ast.submitted.action)) {
    return {
      status: 'CoreRejected', ast,
      rejection: {
        status: 'Rejected', judgment: 'intent', code: 'S0_INTENT_SCOPE',
        diagnosticWork: 1, publishedPost: null, publishedEffects: null,
      },
    };
  }
  return result.status === 'Rejected'
    ? { status: 'CoreRejected', ast, rejection: result }
    : {
        status: 'PreparedUnqualified', ast, candidate: result,
        unverifiedBindings: ['agreement-id', 'selected-program', 'asset-scale', 'authenticated-predecessor'],
      };
}

```

## COMPLETE actual TS consumer experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts SHA256 855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda
```
/** Provisional MIL/4 S0 preparation. This module never returns ledger admission. */

export const MIL4_S0_CORE = 'moriarty-core/5' as const;
export const MIL4_S0_INTENT = 'moriarty-intent/3' as const;
export const MIL4_S0_SOURCE = 'moriarty-financial-agreement-source/6' as const;
const U128 = (1n << 128n) - 1n;
const S128 = (1n << 127n) - 1n;
const IDENTIFIER = /^[A-Za-z][A-Za-z0-9._-]{0,63}$/;

export interface S0Balance { account: string; amount: string }
export interface S0Allowance { owner: string; remaining: string; spent: string }
export interface S0Obligation {
  id: string; debtor: string; creditor: string; asset: string;
  principal: string; accrued: string; outstanding: string;
  status: 'Outstanding' | 'Settled';
}
export interface S0State {
  core: typeof MIL4_S0_CORE; domain: string; asset: string;
  head: string; round: string; workRemaining: string; workSpent: string;
  balances: S0Balance[]; allowances: S0Allowance[];
  obligations: S0Obligation[]; consumedReplay: string[];
}
interface S0IntentBase {
  version: typeof MIL4_S0_INTENT; core: typeof MIL4_S0_CORE;
  sourceProfile: typeof MIL4_S0_SOURCE; programId: string;
  sourceHash: string; policyDigest: string; signedDigest?: string; keyRef: string;
  domain: string; asset: string; signer: string; nonce: string;
  preHead: string; notBefore: string; notAfter: string;
  grossCap: string; feeCap: string; netFloor: string;
}
export interface S0TransferIntent extends S0IntentBase {
  kind: 'Transfer'; recipient: string; feeRecipient: string;
  amount: string; fee: string;
}
export interface S0RepayIntent extends S0IntentBase {
  kind: 'Repay'; obligationId: string; amount: string;
}
export type S0Intent = S0TransferIntent | S0RepayIntent;

export type S0Effect =
  | { kind: 'Debit'; account: string; asset: string; amount: string }
  | { kind: 'Credit'; account: string; asset: string; amount: string }
  | { kind: 'SetObligation'; id: string; principal: string; accrued: string; outstanding: string; status: 'Outstanding' | 'Settled' }
  | { kind: 'UseAllowance'; owner: string; amount: string }
  | { kind: 'UseReplay'; key: string }
  | { kind: 'AdvanceHead'; predecessor: string; successor: string };

export type S0Judgment = 'stage' | 'intent' | 'effect' | 'authority' | 'history' | 'failure';
export interface S0Rejected {
  status: 'Rejected'; judgment: S0Judgment; code: string;
  diagnosticWork: 1; publishedPost: null; publishedEffects: null;
}
export interface S0PreparedUnqualified {
  status: 'PreparedUnqualified'; core: typeof MIL4_S0_CORE;
  preHead: string; effects: S0Effect[]; candidatePost: S0State;
  requiredPremises: readonly ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'];
}
export type S0Result = S0Rejected | S0PreparedUnqualified;
export interface S0RequestedOutcome {
  phase: 'TerminalSuccess' | 'RequestedFailure';
  retainedEffects: unknown[];
  retainedDuties: unknown[];
}
/** An experiment assumption. Possession of this tuple does not authenticate it. */
export interface S0LocalStipulation {
  intent: S0Intent; state: S0State; round: string; expectedSuccessor: string;
  requestedOutcome: S0RequestedOutcome;
}
const TERMINAL_SUCCESS: S0RequestedOutcome = {
  phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [],
};

function reject(judgment: S0Judgment, code: string): S0Rejected {
  return { status: 'Rejected', judgment, code, diagnosticWork: 1, publishedPost: null, publishedEffects: null };
}
function id(value: unknown): value is string {
  return typeof value === 'string' && IDENTIFIER.test(value);
}
function opaque(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0 && value.length <= 1024
    && !/[\u0000-\u001f\u007f]/.test(value);
}
function uint(value: unknown, max: bigint = U128): bigint | null {
  if (typeof value !== 'string' || !/^(0|[1-9][0-9]*)$/.test(value)) return null;
  const parsed = BigInt(value);
  return parsed <= max ? parsed : null;
}
function distinct<T>(values: T[]): boolean { return new Set(values).size === values.length; }
function stableValue(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(stableValue);
  if (value !== null && typeof value === 'object') {
    const record = value as Record<string, unknown>;
    return Object.fromEntries(Object.keys(record).sort().map((key) => [key, stableValue(record[key])]));
  }
  return value;
}
function sameValue(left: unknown, right: unknown): boolean {
  return JSON.stringify(stableValue(left)) === JSON.stringify(stableValue(right));
}
function replayId(value: unknown): value is string {
  if (typeof value !== 'string') return false;
  try {
    const parts: unknown = JSON.parse(value);
    return Array.isArray(parts) && parts.length === 3
      && id(parts[0]) && id(parts[1]) && opaque(parts[2])
      && JSON.stringify(parts) === value;
  } catch { return false; }
}
function boundedAdd(a: bigint, b: bigint): bigint | null {
  const sum = a + b;
  return sum <= U128 ? sum : null;
}
function sameEffects(expected: S0Effect[], supplied: unknown): boolean {
  if (!Array.isArray(supplied) || supplied.length !== expected.length) return false;
  return expected.every((line, index) => {
    const got = supplied[index];
    if (got === null || typeof got !== 'object' || Array.isArray(got)) return false;
    const a = line as unknown as Record<string, unknown>;
    const b = got as Record<string, unknown>;
    return Object.keys(a).length === Object.keys(b).length
      && Object.keys(a).every((key) => JSON.stringify(a[key]) === JSON.stringify(b[key]));
  });
}

/**
 * Prepare a complete local candidate against supplied state.
 * The state, signature and ledger head are not authenticated by this function.
 * A localStipulation is a finite-comparison assumption supplied by the caller;
 * matching it does not authenticate any external fact or qualify the result.
 */
export function prepareMil4S0(
  state: S0State,
  intent: S0Intent,
  submittedEffects: unknown,
  proposedPostHead: string,
  requestedOutcome: S0RequestedOutcome = TERMINAL_SUCCESS,
  localStipulation?: S0LocalStipulation | null,
): S0Result {
  if (state?.core !== MIL4_S0_CORE || intent?.core !== MIL4_S0_CORE
      || intent?.version !== MIL4_S0_INTENT || intent?.sourceProfile !== MIL4_S0_SOURCE
      || (intent?.kind !== 'Transfer' && intent?.kind !== 'Repay')
      || (intent?.kind === 'Transfer' && intent?.programId !== 'TransferLiteralFee')
      || (intent?.kind === 'Repay' && intent?.programId !== 'RepayAccrualFirst')
      || !id(state.domain) || !id(state.asset)
      || !opaque(state.head) || !id(intent.domain) || !id(intent.asset)
      || !Array.isArray(state.balances) || !Array.isArray(state.allowances)
      || !Array.isArray(state.obligations) || !Array.isArray(state.consumedReplay)
      || uint(state.round) === null || uint(state.workRemaining) === null
      || uint(state.workSpent) === null
      || !requestedOutcome || typeof requestedOutcome !== 'object'
      || !['TerminalSuccess', 'RequestedFailure'].includes(requestedOutcome.phase)
      || !Array.isArray(requestedOutcome.retainedEffects)
      || !Array.isArray(requestedOutcome.retainedDuties)
      || boundedAdd(BigInt(state.workRemaining), BigInt(state.workSpent)) === null) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (state.balances.some((v) => !v || typeof v !== 'object')
      || state.allowances.some((v) => !v || typeof v !== 'object')
      || state.obligations.some((v) => !v || typeof v !== 'object')
      || !distinct(state.balances.map((v) => v.account))
      || !distinct(state.allowances.map((v) => v.owner))
      || !distinct(state.obligations.map((v) => v.id))
      || !distinct(state.consumedReplay)
      || state.balances.some((v) => !id(v.account) || uint(v.amount) === null)
      || state.allowances.some((v) => !id(v.owner) || uint(v.remaining) === null || uint(v.spent) === null
        || boundedAdd(BigInt(v.remaining), BigInt(v.spent)) === null)
      || state.obligations.some((v) => !id(v.id) || !id(v.debtor) || !id(v.creditor) || !id(v.asset)
        || uint(v.principal, S128) === null || uint(v.accrued, S128) === null
        || uint(v.outstanding, S128) === null
        || !['Outstanding', 'Settled'].includes(v.status)
        || BigInt(v.principal) + BigInt(v.accrued) !== BigInt(v.outstanding)
        || (v.status === 'Settled') !== (v.outstanding === '0'))
      || state.consumedReplay.some((v) => !replayId(v))) return reject('stage', 'S0_STAGE_UNSUPPORTED');

  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient) && id(intent.feeRecipient)
      && (!state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === intent.recipient)
        || !state.balances.some((v) => v.account === intent.feeRecipient)
        || !state.allowances.some((v) => v.owner === intent.signer)
        || state.obligations.length !== 0
        || state.allowances.length !== 1)) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient)
      && id(intent.feeRecipient)
      && distinct([intent.signer, intent.recipient, intent.feeRecipient])
      && (state.balances.length !== 3 || state.obligations.length !== 0
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== intent.recipient
        || state.balances[2].account !== intent.feeRecipient
        || state.allowances.length !== 1
        || state.allowances[0].owner !== intent.signer)) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Repay' && id(intent.signer) && id(intent.obligationId)) {
    const debt = state.obligations.find((v) => v.id === intent.obligationId);
    if (!debt || debt.asset !== state.asset || debt.debtor !== intent.signer
        || debt.creditor === intent.signer
        || debt.status !== 'Outstanding'
        || !state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === debt.creditor)
        || !state.allowances.some((v) => v.owner === intent.signer)
        || state.balances.length !== 2 || state.obligations.length !== 1
        || state.allowances.length !== 1
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== debt.creditor
        || state.allowances[0].owner !== intent.signer) {
      return reject('stage', 'S0_STAGE_UNSUPPORTED');
    }
  }
  if (localStipulation !== undefined && (localStipulation === null
      || !sameValue(localStipulation.intent, intent)
      || !sameValue(localStipulation.state, state)
      || localStipulation.round !== state.round
      || !opaque(localStipulation.expectedSuccessor)
      || !sameValue(localStipulation.requestedOutcome, requestedOutcome))) {
    return reject('stage', 'S0_STAGE_PREMISE');
  }

  const notBefore = uint(intent.notBefore);
  const notAfter = uint(intent.notAfter);
  const grossCap = uint(intent.grossCap, S128);
  const feeCap = uint(intent.feeCap, S128);
  const netFloor = uint(intent.netFloor, S128);
  if (!id(intent.programId) || !opaque(intent.sourceHash) || !opaque(intent.policyDigest)
      || (intent.signedDigest !== undefined && !opaque(intent.signedDigest)) || !opaque(intent.keyRef)
      || !id(intent.signer) || !opaque(intent.nonce) || !opaque(intent.preHead)
      || intent.domain !== state.domain || intent.asset !== state.asset
      || notBefore === null || notAfter === null || notBefore > notAfter
      || grossCap === null || feeCap === null || netFloor === null
      || BigInt(state.round) < notBefore || BigInt(state.round) > notAfter) {
    return reject('intent', 'S0_INTENT_SCOPE');
  }

  const replayKey = JSON.stringify([state.domain, intent.signer, intent.nonce]);
  const balances = state.balances.map((v) => ({ ...v }));
  const allowances = state.allowances.map((v) => ({ ...v }));
  const obligations = state.obligations.map((v) => ({ ...v }));
  const ownerBalance = balances.find((v) => v.account === intent.signer);
  const ownerAllowance = allowances.find((v) => v.owner === intent.signer);
  let gross: bigint;
  let effects: S0Effect[];

  if (intent.kind === 'Transfer') {
    const v = uint(intent.amount, S128);
    const fee = uint(intent.fee, S128);
    if (!id(intent.recipient) || !id(intent.feeRecipient) || v === null || fee === null || v === 0n) {
      return reject('intent', 'S0_INTENT_SCOPE');
    }
    gross = v + fee;
    if (fee > feeCap || gross > grossCap || v < netFloor) return reject('intent', 'S0_INTENT_SCOPE');
    if (!distinct([intent.signer, intent.recipient, intent.feeRecipient])) {
      return reject('intent', 'S0_INTENT_ALIAS');
    }
    const recipient = balances.find((row) => row.account === intent.recipient);
    const feeRecipient = balances.find((row) => row.account === intent.feeRecipient);
    if (!ownerBalance || !recipient || !feeRecipient) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < gross
        || boundedAdd(BigInt(recipient.amount), v) === null
        || (fee > 0n && boundedAdd(BigInt(feeRecipient!.amount), fee) === null)) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    ownerBalance.amount = (BigInt(ownerBalance.amount) - gross).toString();
    recipient.amount = (BigInt(recipient.amount) + v).toString();
    if (fee > 0n) feeRecipient!.amount = (BigInt(feeRecipient!.amount) + fee).toString();
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: gross.toString() },
      { kind: 'Credit', account: intent.recipient, asset: state.asset, amount: v.toString() },
      ...(fee > 0n ? [{ kind: 'Credit' as const, account: intent.feeRecipient, asset: state.asset, amount: fee.toString() }] : []),
    ];
  } else if (intent.kind === 'Repay') {
    const n = uint(intent.amount, S128);
    if (!id(intent.obligationId) || n === null || n === 0n) return reject('intent', 'S0_INTENT_SCOPE');
    const obligation = obligations.find((row) => row.id === intent.obligationId);
    if (!obligation || obligation.asset !== state.asset || obligation.debtor !== intent.signer
        || obligation.status !== 'Outstanding') return reject('stage', 'S0_STAGE_UNSUPPORTED');
    const p = BigInt(obligation.principal);
    const a = BigInt(obligation.accrued);
    gross = n;
    if (gross > grossCap || feeCap !== 0n || netFloor !== 0n) return reject('intent', 'S0_INTENT_SCOPE');
    if (n > BigInt(obligation.outstanding)) return reject('effect', 'S0_EFFECT_RANGE');
    const creditor = balances.find((row) => row.account === obligation.creditor);
    if (!ownerBalance || !creditor) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < n || boundedAdd(BigInt(creditor.amount), n) === null) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    const da = n < a ? n : a;
    const dp = n - da;
    const afterP = p - dp;
    const afterA = a - da;
    const afterOutstanding = afterP + afterA;
    ownerBalance.amount = (BigInt(ownerBalance.amount) - n).toString();
    creditor.amount = (BigInt(creditor.amount) + n).toString();
    obligation.principal = afterP.toString();
    obligation.accrued = afterA.toString();
    obligation.outstanding = afterOutstanding.toString();
    obligation.status = afterOutstanding === 0n ? 'Settled' : 'Outstanding';
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: n.toString() },
      { kind: 'Credit', account: obligation.creditor, asset: state.asset, amount: n.toString() },
      { kind: 'SetObligation', id: obligation.id, principal: obligation.principal,
        accrued: obligation.accrued, outstanding: obligation.outstanding, status: obligation.status },
    ];
  } else return reject('stage', 'S0_STAGE_UNSUPPORTED');

  effects.push(
    { kind: 'UseAllowance', owner: intent.signer, amount: gross.toString() },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: intent.preHead, successor: proposedPostHead },
  );
  if (!sameEffects(effects, submittedEffects)) {
    return reject('effect', 'S0_EFFECT_MISMATCH');
  }
  if (!ownerAllowance || BigInt(ownerAllowance.remaining) < gross
      || boundedAdd(BigInt(ownerAllowance.spent), gross) === null
      || BigInt(state.workRemaining) < 1n || boundedAdd(BigInt(state.workSpent), 1n) === null) {
    return reject('authority', 'S0_AUTH_SCOPE');
  }
  ownerAllowance.remaining = (BigInt(ownerAllowance.remaining) - gross).toString();
  ownerAllowance.spent = (BigInt(ownerAllowance.spent) + gross).toString();
  if (intent.preHead !== state.head) return reject('history', 'S0_HISTORY_STALE');
  if (state.consumedReplay.includes(replayKey)) return reject('history', 'S0_HISTORY_REPLAY');
  if (!opaque(proposedPostHead) || proposedPostHead === state.head) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (localStipulation !== undefined
      && proposedPostHead !== localStipulation!.expectedSuccessor) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (requestedOutcome.phase !== 'TerminalSuccess'
      || requestedOutcome.retainedEffects.length !== 0
      || requestedOutcome.retainedDuties.length !== 0) {
    return reject('failure', 'S0_FAILURE_UNSUPPORTED');
  }

  return {
    status: 'PreparedUnqualified', core: MIL4_S0_CORE, preHead: state.head, effects,
    candidatePost: {
      ...state, balances, allowances, obligations,
      consumedReplay: [...state.consumedReplay, replayKey],
      head: proposedPostHead,
      workRemaining: (BigInt(state.workRemaining) - 1n).toString(),
      workSpent: (BigInt(state.workSpent) + 1n).toString(),
    },
    requiredPremises: ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'],
  };
}

```

## COMPLETE actual public Rust->TS regression SHA256 ea420fd5230700103762acfb8de3270bf46a1f6b49a6257cefab10d1ba39fc27
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

## Actual4step result receipt (compact display; byte-identical original physical)
```json
{"finishedAt":"2026-10-18T10:24:37.482116+00:00","steps":[{"name":"library-transfer","finishedAt":"2026-10-18T10:23:36.462804+00:00","exitCode":0,"wallSeconds":20.744100980999065,"stdoutSha256":"d3aeee1d2f5c14cde13398a572ab5d82a23684634ef362af7a87c6884ce1363e","stderrSha256":"74ba17fa151b9d9fcba20a9feade4615afdf3c799767f0b641dbe105c6fd2459","sourceCommitAfter":"8267ad950742c94f74fd871c266fa995c4e9e893","sourceTreeAfter":"a936ca2a3383045b38e90cc3ab867ae948fa64e1","sourceStatusAfter":"","attemptConsumed":true},{"name":"cli-source6-expand","finishedAt":"2026-10-18T10:24:24.282371+00:00","exitCode":0,"wallSeconds":47.75215807800123,"stdoutSha256":"9a2d88567d100065f162b004ba13e06ec8044f1f9a0cb8d99621b46d4b3fa30b","stderrSha256":"e9f3f8dfe0cc1589a0ee1f3fbc429882e5c82cfdfda9d82aac51f22edf1e6b68","sourceCommitAfter":"8267ad950742c94f74fd871c266fa995c4e9e893","sourceTreeAfter":"a936ca2a3383045b38e90cc3ab867ae948fa64e1","sourceStatusAfter":"","attemptConsumed":true},{"name":"build-public-binary","finishedAt":"2026-10-18T10:24:36.899330+00:00","exitCode":0,"wallSeconds":12.539828752996982,"stdoutSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855","stderrSha256":"18680af64cd36d8850d162a4075ec70eb0e72b68c8be06475fe34feda144aa30","sourceCommitAfter":"8267ad950742c94f74fd871c266fa995c4e9e893","sourceTreeAfter":"a936ca2a3383045b38e90cc3ab867ae948fa64e1","sourceStatusAfter":"","attemptConsumed":true},{"name":"node-public-financial-consumer","finishedAt":"2026-10-18T10:24:37.352903+00:00","exitCode":0,"wallSeconds":0.21350831399831804,"stdoutSha256":"88af44f8c3873052422270dd58473c925d07fdf2aa7d049f3976f2f05f1caebe","stderrSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855","sourceCommitAfter":"8267ad950742c94f74fd871c266fa995c4e9e893","sourceTreeAfter":"a936ca2a3383045b38e90cc3ab867ae948fa64e1","sourceStatusAfter":"","attemptConsumed":true}],"allFourCompleted":true,"allCompletedPassed":true,"sourceCommit":"8267ad950742c94f74fd871c266fa995c4e9e893","sourceTree":"a936ca2a3383045b38e90cc3ab867ae948fa64e1","ownedTemporaryRoot":"/tmp/doctorwatson-source6-public-consumer-j73iseob","temporaryRootExistsAfter":false,"scope":"SelectedordinarySource6library/CLIandactualRuststdout-toTSprovisionalfinancialconsumer only. NoSDK/proof/native/ledgerqualification; fullwire/work/postworkoracles unasserted."}
```

## ACTUAL stdout library-transfer SHA256 d3aeee1d2f5c14cde13398a572ab5d82a23684634ef362af7a87c6884ce1363e
```

running 3 tests
test literal_decimal_transfer_has_complete_source6_presentation ... ok
test referenced_quantity_sum_and_fee_use_exact_atomic_values ... ok
test unknown_action_diagnostic_escapes_control_and_non_ascii_characters ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


```

## ACTUAL stdout cli-source6-expand SHA256 9a2d88567d100065f162b004ba13e06ec8044f1f9a0cb8d99621b46d4b3fa30b
```

running 7 tests
test beta_default_retains_core_parser_refusal_boundary ... ok
test checked_beta_reaches_real_expansion_library_and_raw_stdout ... ok
test core_default_retains_complete_program_bytes ... ok
test diagnostic_presentation_escapes_filename_action_and_scenario_shape_errors ... ok
test explicit_source6_quiet_output_is_raw_source6_text ... ok
test scenario_profile_and_subset_have_explicit_consumer_diagnostics ... ok
test source_action_selection_precedes_scenario_file_read ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


```

## ACTUAL stdout node-public-financial-consumer SHA256 88af44f8c3873052422270dd58473c925d07fdf2aa7d049f3976f2f05f1caebe
```
✔ invoice: public Rust output through provisional real Source/6/Core/5 consumer (16.029457ms)
✔ workshop: public Rust output through provisional real Source/6/Core/5 consumer (6.122941ms)
ℹ tests 2
ℹ suites 0
ℹ pass 2
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 181.616641

```

## node-public-financial-consumer/BINARY-IDENTITY.json (compact exact receipt)
```json
{"path":"/tmp/doctorwatson-source6-public-consumer-j73iseob/target/debug/mori","sha256":"558b43b66bb1adfae4878460b54b20cb82e88f5ca0717f7b7e7ffdca31344e2b","bytes":82397248,"sourceCommit":"8267ad950742c94f74fd871c266fa995c4e9e893","sourceTree":"a936ca2a3383045b38e90cc3ab867ae948fa64e1","nodePath":"/usr/local/bin/node","nodeSha256":"7fde7b8afa198da66257f42ee2001d874c7355631e6d1579a5fb5ef1f246df4c"}
```

## node-observations/run.json (compact exact receipt)
```json
{"node":"v24.21.0","binary":"/tmp/doctorwatson-source6-public-consumer-j73iseob/target/debug/mori","binarySha256":"558b43b66bb1adfae4878460b54b20cb82e88f5ca0717f7b7e7ffdca31344e2b","financialFreezeSha256":"64a6b4a6e454bbc6bc2cf3582581e81e3dd5fd379f9ed3fc89bbfc8051180269","qualification":"local-stipulation-only; provisional S0; no runtime qualification beyond recorded checks","unasserted":["whole emitted wire golden","success work policy/cost/post counters","authentication","native embedding","proof","ledger","parent acceptance"]}
```

## ACTUAL unmodified public stdout invoice SHA256 32015ccfa3494f4592a041d624f9f19e2ef11eaeb446065d520fc0e9cde28913
```
profile "moriarty-financial-agreement-source/6";
agreement Invoice {
domain Midnight;
settlement A scale 2;
selected TransferLiteralFee source_hash "src1" digest "policy1";
intent {
signer Owner key "key1";
nonce "n1";
pre_head "h0";
valid 0..10;
gross_cap 1010;
fee_cap 10;
net_floor 1000;
failure success_only;
signed_action transfer from Owner to Recipient fee_to Fee value 1000 fee 10;
observations empty;
disclosures empty;
retained_effects empty;
retained_duties empty;
delegation none;
recovery none;
}
authenticated {
head "h0";
predecessor "genesis";
round 1;
balance Owner 10000;
balance Recipient 0;
balance Fee 0;
allowance Owner remaining 10000 spent 0;
replay unused;
work_remaining 10;
work_spent 0;
}
submit transfer from Owner to Recipient fee_to Fee value 1000 fee 10;
effects {
debit Owner 1010;
credit Recipient 1000;
credit Fee 10;
use_allowance Owner 1010;
use_replay "n1";
advance_head "h0" "h1";
}
post_head "h1";
}

```

## ACTUAL captured Node local outcome invoice (compact status/candidate/bindings projection; full AST+lowered+outcome original physical)
```json
{"status":"PreparedUnqualified","candidate":{"status":"PreparedUnqualified","core":"moriarty-core/5","preHead":"h0","effects":[{"kind":"Debit","account":"Owner","asset":"A","amount":"1010"},{"kind":"Credit","account":"Recipient","asset":"A","amount":"1000"},{"kind":"Credit","account":"Fee","asset":"A","amount":"10"},{"kind":"UseAllowance","owner":"Owner","amount":"1010"},{"kind":"UseReplay","key":"[\"Midnight\",\"Owner\",\"n1\"]"},{"kind":"AdvanceHead","predecessor":"h0","successor":"h1"}],"candidatePost":{"core":"moriarty-core/5","domain":"Midnight","asset":"A","head":"h1","round":"1","workRemaining":"9","workSpent":"1","balances":[{"account":"Owner","amount":"8990"},{"account":"Recipient","amount":"1000"},{"account":"Fee","amount":"10"}],"allowances":[{"owner":"Owner","remaining":"8990","spent":"1010"}],"obligations":[],"consumedReplay":["[\"Midnight\",\"Owner\",\"n1\"]"]},"requiredPremises":["canonical-intent-signature","snapshot-to-head","head-extension","atomic-ledger-compare-and-consume"]},"unverifiedBindings":["agreement-id","selected-program","asset-scale","authenticated-predecessor"]}
```

## ACTUAL unmodified public stdout workshop SHA256 635a717b2a026c3afff4a8df45c0cf7cf3b02f2ea02a625d83fff1f7c1eed187
```
profile "moriarty-financial-agreement-source/6";
agreement WorkshopReceipt {
domain Midnight;
settlement A scale 2;
selected TransferLiteralFee source_hash "src2" digest "policy2";
intent {
signer Owner key "key2";
nonce "n2";
pre_head "s0";
valid 2..20;
gross_cap 242;
fee_cap 7;
net_floor 235;
failure success_only;
signed_action transfer from Owner to Recipient fee_to Fee value 235 fee 7;
observations empty;
disclosures empty;
retained_effects empty;
retained_duties empty;
delegation none;
recovery none;
}
authenticated {
head "s0";
predecessor "workshop_prior";
round 5;
balance Owner 1000;
balance Recipient 25;
balance Fee 3;
allowance Owner remaining 1000 spent 17;
replay unused;
work_remaining 12;
work_spent 4;
}
submit transfer from Owner to Recipient fee_to Fee value 235 fee 7;
effects {
debit Owner 242;
credit Recipient 235;
credit Fee 7;
use_allowance Owner 242;
use_replay "n2";
advance_head "s0" "s1";
}
post_head "s1";
}

```

## ACTUAL captured Node local outcome workshop (compact status/candidate/bindings projection; full AST+lowered+outcome original physical)
```json
{"status":"PreparedUnqualified","candidate":{"status":"PreparedUnqualified","core":"moriarty-core/5","preHead":"s0","effects":[{"kind":"Debit","account":"Owner","asset":"A","amount":"242"},{"kind":"Credit","account":"Recipient","asset":"A","amount":"235"},{"kind":"Credit","account":"Fee","asset":"A","amount":"7"},{"kind":"UseAllowance","owner":"Owner","amount":"242"},{"kind":"UseReplay","key":"[\"Midnight\",\"Owner\",\"n2\"]"},{"kind":"AdvanceHead","predecessor":"s0","successor":"s1"}],"candidatePost":{"core":"moriarty-core/5","domain":"Midnight","asset":"A","head":"s1","round":"5","workRemaining":"11","workSpent":"5","balances":[{"account":"Owner","amount":"758"},{"account":"Recipient","amount":"260"},{"account":"Fee","amount":"10"}],"allowances":[{"owner":"Owner","remaining":"758","spent":"259"}],"obligations":[],"consumedReplay":["[\"Midnight\",\"Owner\",\"n2\"]"]},"requiredPremises":["canonical-intent-signature","snapshot-to-head","head-extension","atomic-ledger-compare-and-consume"]},"unverifiedBindings":["agreement-id","selected-program","asset-scale","authenticated-predecessor"]}
```

## Independent expectations and exact scope
Original20payload financial freeze64a6b4a6e454bbc6bc2cf3582581e81e3dd5fd379f9ed3fc89bbfc8051180269 and20payload nominal freeze bdd550f9b32b94919e67caa3d444ef0f5ea583903cbc85ec38513f1451bd6107 are physical, unchanged, independently specified BEFORE actual consumption. Invoice gross1010,value1000,fee10; local conditional post balances8990/1000/10, allowance8990/1010. NEW chosen Workshop gross242,value235,fee7; post758/260/10, allowance758/259, preserving nonzero recipient/fee openings and spent counters. Source6 selected/Core programId=TransferLiteralFee is distinct from beta actionpay/intentPayment/agreement; bare Source6 replaynonce lowers to semantic(domain,signer,nonce). Frozen nominal facts impose economic IDs/claims/scale/round/work-opening read preservation, but exact agreement token/full wire/DTO/origin recipe and independent success-work schedule/post counters remain unasserted. Real12tests=3library+7CLI+2publicRuststdout->TS; binarybuild is a4thcommand, not another test. Candidate test assertions compare financial/input projection; they do not directly test all124 nominal coverage fields. A separate postcapture comparison, if physically included, is distinct from these12tests, not new runtime or newly derived oracle.
Core/5 local preparation remains provisional PreparedUnqualified with external signature/snapshot/head-extension/compare-consume premises and unverified agreement-id/selected-program/asset-scale/authenticated-predecessor bindings. Actual captured work values are observations, not adopted/independently certified work policy. No beta-to-native embedding, signed authorization, SDK/native/proof/VM/key/ledger/target result follows. All U0–U7/M1–M17/MC/SP/G, finance/composition/native/target exits remain open. UNI018 names exact profile/consumer/subset; UNI019 confines correspondence to justified fields; UNI020 requires independent complete financial/work observations and retains missing subjects; UNI021 separates child result from parent acceptance. Full normative documents are physical.

## COMPLETE additional postcapture comparison included BEFORE this dispatch
Physical postcapture-nominal-comparison/ contains COMPLETE124row report, declared target/Core comparison, ADAPTERS.md and full compare_public.py, with inventory513c9e36e77e8cde9ab06434c38a1194a778fd96361c3d499cb09cd04fb9391d and FREEZE81d43a6c7e35093efed39648b1c30277cdccb5fed7d381410ea56bf7ec23164d. Its observed input hashes match these physical captures; original independent nominal bdd550 freeze remains unchanged. Reported counts:124coverage rows104matched/20explicitUNASSERTED/0missing/0mismatch; target74matched; Core52matched/4UNASSERTED; supplemental10matched. Review the complete comparison/adapters/source as source/result evidence, not a new oracle/runtime/trust judgment. These overlapping report groups are not extra tests and must not be summed into a new acceptance denominator. Qualification/postwork, exactagreement/fullwire/metadata/origin/native/proof/ledger remain limited as declared.
