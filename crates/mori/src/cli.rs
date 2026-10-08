use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::commands::{
    check, expand, format, init, inspect, intent, lsp, mcp, simulate, test, verify_intent,
};
use crate::shell::Verbosity;
use crate::styles;

/// Moriarty: a financial agreement language for Midnight.
#[derive(Debug, Parser)]
#[command(name = "mori", version, styles = styles::CLAP)]
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
    /// Format source files in place
    #[command(visible_alias = "fmt")]
    Format(format::Args),
    /// Show identity claims, operative bounds and open premises
    Inspect(inspect::Args),
    /// Expand an action into Source/6 with an input origin map
    Expand(expand::Args),
    /// Run an action locally against a scenario
    Simulate(simulate::Args),
    /// Run the test cases in a project directory
    Test(test::Args),
    /// Produce the owner signing bytes for an action
    Intent(intent::Args),
    /// Verify a signature, then prepare the action locally
    VerifyIntent(verify_intent::Args),
    /// Run the language server over stdio
    Lsp(lsp::Args),
    /// Run the read-only MCP server over stdio
    Mcp(mcp::Args),
}
