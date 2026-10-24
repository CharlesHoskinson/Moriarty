use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::cli_args::simulate;
use crate::commands::{
    capabilities, check, compile, compile_call, expand, format, init, inspect, intent, keygen_call,
    keygen_signed_call, lsp, mcp, prepare_call, prepare_signed_call, prove, prove_call,
    prove_signed_call, replay, test, verify, verify_call, verify_intent, verify_signed_call,
};
use crate::shell::Verbosity;
use crate::styles;

/// Moriarty: a financial agreement language for Midnight.
#[derive(Debug, Parser)]
#[command(name = "mori", version, styles = styles::CLAP)]
#[cfg_attr(
    all(not(feature = "native"), not(feature = "source6-expansion")),
    command(
        after_help = "Built without native: local Core reference tooling and explicit Sherlock host preparation/simulation. Midnight compile, call, proof, targeted inspect, trace/replay and JubJub profiles require the native feature."
    )
)]
#[cfg_attr(
    all(not(feature = "native"), feature = "source6-expansion"),
    command(
        after_help = "Built without native: local Core reference tooling, explicit Sherlock host preparation/simulation and explicit Source/6 transfer expansion. Midnight compile, call, proof, targeted inspect, trace/replay and JubJub profiles require the native feature."
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

pub use crate::cli_args::SignedProfile;

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
    /// Compile Core/1 or Core/2 ZKIRv3 circuits or prepare an explicit Sherlock host artifact
    ///
    /// Native routes use a language "moriarty/1"; or language "moriarty/2"; header.
    /// Sherlock host preparation requires --execution-profile and explicit source selection.
    /// Beta/1 and Source/6 text are not compiler inputs.
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, explicit Sherlock host preparation is available; Core/1 and Core/2 ZKIR compilation require the native feature."
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
    /// Show Core/1 IR or explicitly emit beta Source/6 transfer text
    ///
    /// Default expansion accepts moriarty/1 Core source. The explicit
    /// --emit-source6-transfer option emits a local beta transfer proposal.
    Expand(expand::Args),
    /// Run a Core/1 action, Core/2 stage or explicitly selected Sherlock entry locally
    ///
    /// Core routes use a language "moriarty/1"; or language "moriarty/2"; header.
    /// Sherlock routes require --execution-profile and an explicit source selection.
    /// moriarty-beta/1 and Source/6 text are not simulation inputs.
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, local Core simulation, staged start/resume and explicit Sherlock host simulation are available; --trace requires the native feature."
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

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    #[test]
    fn help_distinguishes_core_consumers_from_source6_proposals() {
        let command = Cli::command();
        let compile = command.find_subcommand("compile").unwrap();
        let compile_help = compile.get_long_about().unwrap().to_string();
        for header in [r#"language "moriarty/1";"#, r#"language "moriarty/2";"#] {
            assert!(compile_help.contains(header), "{compile_help}");
        }
        assert!(
            compile_help.contains("Beta/1 and Source/6 text are not compiler inputs."),
            "{compile_help}"
        );

        let simulate = command.find_subcommand("simulate").unwrap();
        let simulate_help = simulate.get_long_about().unwrap().to_string();
        for profile in ["moriarty/1", "moriarty/2"] {
            assert!(simulate_help.contains(profile), "{simulate_help}");
        }
        assert!(
            simulate_help.contains("moriarty-beta/1 and Source/6 text are not simulation inputs."),
            "{simulate_help}"
        );

        let expand = command.find_subcommand("expand").unwrap();
        let expand_help = expand.get_long_about().unwrap().to_string();
        assert!(expand_help.contains("moriarty/1"), "{expand_help}");
        let source6 = expand
            .get_arguments()
            .find(|argument| argument.get_id().as_str() == "emit_source6_transfer")
            .unwrap();
        let source6_help = source6.get_long_help().unwrap().to_string();
        for boundary in [
            "moriarty-beta/1",
            "local-stipulation-only",
            "Source/6 output is not compiled or simulated by this option.",
        ] {
            assert!(source6_help.contains(boundary), "{source6_help}");
        }
    }
}
